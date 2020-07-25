# Running our first test ROM

NES dev community has created large suites of tests that can be used to check our emulator. https://wiki.nesdev.com/w/index.php/Emulator_tests

They cover pretty much every aspect, including quirks and famous bugs that were embedded in the platform. 

 <div style="text-align:center"><img src="./images/ch5.1/image_1_i_am_error.png" width="40%"/></div>

We will start with the most basic test that covers main CPU features: instruction set, memory access and CPU cycles. 

The iNES file of the test is located here: [nestest.nes](http://nickmass.com/images/nestest.nes)
The test is also accompanied with an execution log, that show how our platform should behave executing this test: [nestest.log](https://www.qmtpro.com/~nes/misc/nestest.log)

The challenge is to create an execution log feature for our platform. 

 <div style="text-align:center"><img src="./images/ch5.1/image_2_log_structure.png" width="80%"/></div>


For now we can ignore the last column and focus on the first 5. 

The fourth column ```@ 80 = 0200 = 00``` is somewhat interesting. 
* The first number is the actual mem reference that we get if we apply an offset to the requesting address. E1 is using "Indirect X" addressing mode, thus our offset is defined by register X 
* The second number is a 2 byte  target address that was fetched from [0x80 .. 0x81]
* The third number is a content of address cell 0x0200

We already have a place to intercept CPU execution: 

```rust
impl CPU  {

// ..
    pub fn run_with_callback<F>(&mut self, mut callback: F)
   where
       F: FnMut(&mut CPU),
   {
       let ref opcodes: HashMap<u8, &'static opcodes::OpCode> = *opcodes::OPCODES_MAP;
 
       loop {
           callback(self);
 // ...
      }
   }
}
```

All we need to do is to create a callback function that will trace CPU state:


```rust
fn main() {
//...
    cpu.run_with_callback(move |cpu| {
       println!("{}", trace(cpu));
   }
}

```

It's important to get the execution log format exactly as the one that was used in the provided log.

Following tests can help you to get it right:

```rust

#[cfg(test)]
mod test {
   use super::*;
   use crate::bus::Bus;
   use crate::cartridge::test::test_rom;
 
   #[test]
   fn test_format_trace() {
       let mut bus = Bus::new(test_rom());
       bus.mem_write(100, 0xa2);
       bus.mem_write(101, 0x01);
       bus.mem_write(102, 0xca);
       bus.mem_write(103, 0x88);
       bus.mem_write(104, 0x00);
 
       let mut cpu = CPU::new(bus);
       cpu.program_counter = 0x64;
       cpu.register_a = 1;
       cpu.register_x = 2;
       cpu.register_y = 3;
       let mut result: Vec<String> = vec![];
       cpu.run_with_callback(|cpu| {
           result.push(trace(cpu));
       });
       assert_eq!(
           "0064  A2 01     LDX #$01                        A:01 X:02 Y:03 P:24 SP:FD",
           result[0]
       );
       assert_eq!(
           "0066  CA        DEX                             A:01 X:01 Y:03 P:24 SP:FD",
           result[1]
       );
       assert_eq!(
           "0067  88        DEY                             A:01 X:00 Y:03 P:26 SP:FD",
           result[2]
       );
   }
 
   #[test]
   fn test_format_mem_access() {
       let mut bus = Bus::new(test_rom());
       // ORA ($33), Y
       bus.mem_write(100, 0x11);
       bus.mem_write(101, 0x33);
 
 
       //data
       bus.mem_write(0x33, 00);
       bus.mem_write(0x34, 04);
 
       //target cell
       bus.mem_write(0x400, 0xAA);
 
       let mut cpu = CPU::new(bus);
       cpu.program_counter = 0x64;
       cpu.register_y = 0;
       let mut result: Vec<String> = vec![];
       cpu.run_with_callback(|cpu| {
           result.push(trace(cpu));
       });
       assert_eq!(
           "0064  11 33     ORA ($33),Y = 0400 @ 0400 = AA  A:00 X:00 Y:00 P:24 SP:FD",
           result[0]
       );
   }
}
 
```

Now it's time to compare our execution log with the golden standard.


```bash 
cargo run > mynes.log
diff -y mynes.log nestest.log
```

> You can use any diff tool you'd like. But because our NES doesn't support CPU clock cycles yet, it makes sense to remove last columns in the provided log:
> ```bash 
> cat nestest.log | awk '{print substr($0,0, 73)}' > nestest_no_cycle.log
> diff -y mynes.log nestest_no_cycle.log
> ```


If everything is fine, the first mismatch should look like this:

```
C6B3  A9 AA     LDA #$AA                        A:FF X:97 Y:4   C6B3  A9 AA     LDA #$AA                        A:FF X:97 Y:4
C6B5  D0 05     BNE $C6BC                       A:AA X:97 Y:4   C6B5  D0 05     BNE $C6BC                       A:AA X:97 Y:4
C6BC  28        PLP                             A:AA X:97 Y:4   C6BC  28        PLP                             A:AA X:97 Y:4
                                                              > C6BD  04 A9    *NOP $A9 = 00                    A:AA X:97 Y:4
```

I.e. everything that our emulator has produced should exactly match the golden standard. If anything is off before reaching line **0xC6BC**, that means we made a mistake in our CPU implementation. And this needs to be fixed.

But that doesn't explain why our program terminated and why we didn't get the perfect match. 

If you look close enough, our program has failed at 
```bash 
C6BD  04 A9    *NOP $A9 = 00
```

So it looks like our CPU doesn't know that opcode instruction 0x04. 

So here is the bad news: there are about 110 unofficial CPU instructions. And most of the real NES games do use them a lot. In order for us to move on, we would need to implement those. 

The specs can be found here:
* http://nesdev.com/undocumented_opcodes.txt,
* https://wiki.nesdev.com/w/index.php/Programming_with_unofficial_opcodes,
* https://wiki.nesdev.com/w/index.php/CPU_unofficial_opcodes,
* http://www.oxyron.de/html/opcodes02.html 


Remember how to draw an owl? ) 

The testing ROM should drive your progress. At the end, the CPU would support 256 instructions. Given that CPU instructions reserve only 1 byte for the operation code, we've exhausted all possible values. 

At the end, the first mismatch should happen on this line:
```bash
C68B  8D 15 40  STA $4015 = FF                  A:02 X:FF Y:15 P:25 SP:FB
```
almost at the very end of the log file. 

That's a good sign. 4015 is a memory map for the APU register. And we don't have that implemented yet. 
