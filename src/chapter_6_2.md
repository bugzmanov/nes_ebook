# Emulating Interrupts

Interrupts are the mechanism for the CPU to break the sequential execution flow and react to the events that require immediate attention ("attend to an interrupt").

We've already implemented one of the supported interrupts - RESET signal. This interrupt notifies the CPU that a new cartridge was inserted and CPU needs to execute the reset subroutine.

 <div style="text-align:center"><img src="./images/ch6.2/image_4_broadcast_interrupted.png" width="30%"/></div>


PPU communicates that it's entering the VBLANK phase for the frame via another interrupt signal - NMI (Non-Maskable Interrupt). 
From a high-level perspective, this means two things:
- PPU is done rendering current frame
- CPU can safely access PPU memory to update state for the next frame.

> The reason why VBLANK phase is special is that while PPU is rendering visible scan lines it's constantly using internal buffers and memory, external access to IO registers would corrupt data in those buffers and would cause visible graphic glitches on a screen. 

Unlike other interrupts, CPU can't ignore NMI interrupt. And the **Disable Interrupt** flag in the **Status register P** has no effect on the way the CPU responds to NMI.
The CPU however might instruct PPU to not trigger NMI by resetting 7th bit in PPU Control register. 

## Clock cycles

The NMI interrupt is tightly connected to PPU clock cycles:
* the PPU renders 262 scan lines per frame. 
* each scanline lasts for 341 PPU clock cycles 
* upon entering scanline 241, PPU triggers NMI interrupt
* PPU clock cycles is 3 times faster than CPU clock cycles

Nothing beats NESDev wiki in providing [details on line-by-line timing](http://wiki.nesdev.com/w/index.php/PPU_rendering#Line-by-line_timing)

But to simplify, 
 * each PPU frame takes ```341*262=89342 PPU clocks cycles```
 * CPU is guaranteed to receive NMI every interrupt ```~29780 CPU cycles```

> **NOTE:** PPU Cycles and CPU Cycles are not the same thing

On NES Platform all components were running independently in parallel. This makes NES a distributed system. The coordination was carefully designed by game developers based on timing specs of the instructions they were using. I can only imagine how tedious this manual process was. 

The emulator can take multiple approaches to simulate this behaviour:
1) Allocate a thread per component and make simulate proper timing for each instruction. I don't know any emulator that does that. First of all, simulating proper timing is a hell of a task, second of all this approach requires allocating more hardware resources than needed for the task (PPU, CPU and APU would require 3 threads, and potentially would accopy 3 cores on host machine hardware)

2) Execute all components sequentially in one thread, by advancing one clock cycle at a time in each component. This is similar to creating a green-thread runtime and using one dedicated OS thread to run this runtime. It would require heavy investment on creating green-threads runtime on our part. 

3) Execute all components sequentially in one thread, but by letting CPU to execute one full instruction, compute the clock cycles budget for other components. <br/> <br/>For example, if CPU executes "LDA #$01" (op code 0xA9) that means that CPU have spent 2 CPU Cycles, which means that PPU can run for 6 PPU cycles now (PPU clock is ticking 3 times faster than CPU clock)

Because we already have CPU loop mostly spec'd out, the third approach would be the easiest to implement. Granted, it would be the least accurate one. But it's good enough to have something playable as soon as possible.
 
So the flow would look like this: 

 <div style="text-align:center"><img src="./images/ch6.2/image_1_tick_flow.png" width="60%"/></div>

Starting from the CPU:

```rust
impl CPU {
   pub fn run_with_callback<F>(&mut self, mut callback: F)
   where
       F: FnMut(&mut CPU),
   {
      //...
       loop {
        // …
           self.bus.tick(opcode.cycles);
 
           if program_counter_state == self.program_counter {
               self.program_counter += (opcode.len - 1) as u16;
           }
   }
 
   }
}

```

The Bus should keep track of executed cycles and also propagate tick call to PPU, but because PPU clock is 3 times faster than CPU clock, it would muiltiple the value:


```rust
pub struct Bus {
   cpu_vram: [u8; 2048],
   prg_rom: Vec<u8>,
   ppu: NesPPU,
 
   cycles: usize,
}
 
impl Bus {
   pub fn new(rom: Rom) -> Self {
       let ppu = NesPPU::new(rom.chr_rom, rom.screen_mirroring);
 
       Bus {
           cpu_vram: [0; 2048],
           prg_rom: rom.prg_rom,
           ppu: ppu,
           cycles: 0,
       }
   }
   pub fn tick(&mut self, cycles: u8) {
       self.cycles += cycles as usize;
       self.ppu.tick(cycles * 3);
   }
}
```

And then the PPU would track cycles it's in and also which scanline should be drawn according to the current cycle:

```rust
pub struct NesPPU {
   // ...
   scanline: u16,
   cycles: usize,
}
 
 
 
impl NesPPU {
// …
   pub fn tick(&mut self, cycles: u8) -> bool {
       self.cycles += cycles as usize;
       if self.cycles >= 341 {
           self.cycles = self.cycles - 341;
           self.scanline += 1;
 
           if self.scanline == 241 {
               if self.ctrl.generate_vblank_nmi() {
                   self.status.set_vblank_status(true);
                   todo!("Should trigger NMI interrupt")
               }
           }
 
           if self.scanline >= 262 {
               self.scanline = 0;
               self.status.reset_vblank_status();
               return true;
           }
       }
       return false;
   }
}

```

Some crucial details are still missing: some of the operations have variable clock time depending on the execution. 
For example, conditional branch operations (like BNE) take an additional CPU cycle if comparison is successful. And yet another CPU cycle if the JUMP would result program counter to be on another memory page

> Memory page on the NES is 256. For example, the range [0x0000 .. 0x00FF]- belongs to page 0, [0x0100 .. 0x01FF] belongs to page 1, etc.
> To check if 2 addresses are on the same page it's enough to compare the upper byte of the addresses. 

I leave it up to the reader to figure out how to codify those additional ticks that may or may not happen. 

# Interrupts

So far our dependency graph looks straightforward:

 <div style="text-align:center"><img src="./images/ch6.2/image_2_components_dag.png" width="60%"/></div>

The problem is that we need to deliver signal from PPU to CPU and Rust doesn't really allow to have dependency cycles easily. 

One way to overcome this is to replace the push model with pull. The CPU can ask if there are interrupts ready at the beginning of the interpret cycle. 

```rust
impl CPU {
//...
   pub fn run_with_callback<F>(&mut self, mut callback: F)
   where
       F: FnMut(&mut CPU),
   {
       // ...
       loop {
           if let Some(_nmi) = self.bus.poll_nmi_status() {
               self.interrupt_nmi();
           } 
           // …
       }
    }
}
```

The last piece is to implement interrupt behaviour.
Upon receiving the interrupt CPU:
1) finishes execution of current instruction
2) Stores Program Counter and Status flag to the stack
3) Disables Interrupts by setting **Disable Interrupt** flag in status register
4) Loads the Address of Interrupt handler routine from 0xFFFA
5) Sets **Program Counter** register pointing to that address

 <div style="text-align:center"><img src="./images/ch6.2/image_3_interrupt_mem.png" width="40%"/></div>


Interrupt handler would call RTI operation at the end, that would restore Status Flag from stack, and jump back to the saved Program Counter position. Effectively restoring the execution flow of the CPU from where it was left off. 


```rust
   fn interrupt_nmi(&mut self) {
       self.stack_push_u16(self.program_counter);
       let mut flag = self.status.clone();
       flag.set(CpuFlags::BREAK, 0);
       flag.set(CpuFlags::BREAK2, 1);
 
       self.stack_push(flag.bits);
       self.status.insert(CpuFlags::INTERRUPT_DISABLE);
 
       self.bus.tick(2);
       self.program_counter = self.mem_read_u16(0xfffA);
   }
```