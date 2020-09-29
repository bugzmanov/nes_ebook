 # Emulating CPU


The goal of this chapter is to get our first NES game up and running.
We are going to play the Snake game. The source code with comments can be found in [this gist](https://gist.github.com/wkjagt/9043907).

 <div style="text-align:center"><img src="./images/ch3/snk_logo.png" width="40%"/></div>
 <div style="text-align:center"><img src="./images/ch3/snk_game.gif" width="40%"/></div>

CPU is the heart of any computer system. It's the CPUs job to run program instructions and orchestrate all of the available hardware modules to provide the full experience. Despite PPU and APU running their independent circuits, they still have to march under CPUs beat and execute commands issued by the CPU.

Before jumping into implementation, we need to briefly discuss which resources are available to the CPU to do its work.

The only two resources that the CPU has access to are the Memory Map and CPU Registers.

From a programming standpoint, the memory map is just a continuous array of 1-byte cells. NES CPU uses 16-bit for memory addressing, which means that it can address 65536 different memory cells.

As we've seen before, the NES platform had only 2 KiB of RAM connected to the CPU.

 <div style="text-align:center"><img src="./images/ch3/cpu_registers_memory.png" width="80%"/></div>


That RAM is accessible via **[0x0000 … 0x2000]** address space.

Access to **[0x2000 … 0x4020]** is redirected to other available NES hardware modules: PPU, APU, GamePads, etc. (more on this later)

Access to **[0x4020 .. 0x6000]** is a special space that different generations of cartridges used differently. It might be mapped to RAM, ROM, or nothing at all. The space is controlled by so-called mappers - special circuitry on a cartridge. We will ignore this space.

Access to **[0x6000 .. 0x8000]** is reserved to a RAM space on a cartridge if a cartridge has one. It was used in games like Zelda for storing and retrieving the game state. We will ignore this space as well.

Access to **[0x8000 … 0x10000]** is mapped to Program ROM (PRG ROM) space on a cartridge.

Memory access is relatively slow, NES CPU has a few internal memory slots called registers with significantly lower access delay.


> | CPU Operation type  | Execution time (in CPU Cycles)  |
> |---|---|
> | Accessing only registers                         | 2        |
> | Accessing the first 255 bytes of RAM             | 3        |
> | Accessing memory space after the first 255         | 4-7  |


NES CPU has 7 Registers:
* Program Counter (*PC*) - holds the address for the next machine language instruction to be executed.
* Stack Pointer - Memory space [0x0100 .. 0x1FF] is used for stack. The stack pointer holds the address of the top of that space. NES Stack (as all stacks) grows from top to bottom: when a byte gets pushed to the stack, SP register decrements. When a byte is retrieved from the stack, SP register increments.

* Accumulator (*A*) - stores the results of arithmetic, logic, and memory access operations. It used as an input parameter for some operations.

* Index Register X (*X*) - used as an offset in specific memory addressing modes (more on this later). Can be used for auxiliary storage needs (holding temp values, being used as a counter, etc.)

* Index Register Y (*Y*) - similar use cases as register X.

* Processor status (*P*) - 8-bit register represents 7 status flags that can be set or unset depending on the result of the last executed instruction (for example Z flag is set (1) if the result of an operation is 0, and is unset/erased (0) otherwise)


Each CPU comes with a predefined hard-wired instruction set that defines everything a CPU can do.

CPU receives instructions from the application layer in the form of machine codes. And you can think of machine language as a thin layer connecting the software with the hardware.


Full lists of the official 6502 instructions:
* [http://www.obelisk.me.uk/6502/reference.html](http://www.obelisk.me.uk/6502/reference.html)
* [http://www.6502.org/tutorials/6502opcodes.html](http://www.6502.org/tutorials/6502opcodes.html)

I tend to use both of the links. The pages provide full specs of available CPU features and their machine codes.

I highly recommend reading this [interactive tutorial on 6502 instructions](https://skilldrick.github.io/easy6502/) before moving on.

 <div style="text-align:center"><img src="./images/ch3/image_4_opcodes.png" width="80%" /></div>

6502 chip is a relatively simple CPU; it supports only six types of commands and about 64 unique commands. Because some of the instructions have multiple versions for different memory addressing modes, it results in about 150 machine code operations that we are to implement.

> **NOTE:** NES console had a custom chip 2A03 that is based on 6502, but has noticeable differences:
>
> - in addition to official machine operations, it had about 110 unofficial additional opcodes (luckily, about a third of them are No-OPs)
> - it had Audio Processing Unit on-board
> - it didn't support decimal mode for arithmetic
>
> To keep things simple, we would need to implement support for 256 different machine instructions.
>
> The good news is that there are a lot of similarities between instructions. Once we have the foundation in place, we will be constantly reusing them to implement the whole set.
