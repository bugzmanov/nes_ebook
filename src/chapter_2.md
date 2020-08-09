# Understanding the NES Platform
## Achitecture

The simplified architecture of hardware-software interaction looks like this:

<div style="text-align:center"><img src="./images/ch2/image_1_computer_arch.png" width="30%"/></div>

From top to bottom: 
* Applications are running business logic and interact with hardware through an Operating System. 
* An Operating System is communicating with hardware using machine language. 
* On a hardware level, each device can be seen as an array of memory elements or processing units (or both). From this perspective, NES joypad is nothing more than an array of eight 1-bit items, each representing a pressed/released state of a button
* Layers below ALU and Memory elements are less of an interest to us. On a hardware level, it all comes down to logic gates and their arrangements. 

> If you want to get intimate knowledge of how computers are composed, starting from basic principles of boolean logic, I highly recommend the book:<br/> <a href="https://www.goodreads.com/book/show/910789.The_Elements_of_Computing_Systems">"The Elements of Computing Systems. Building a Modern Computer from First Principles"</a> by Noam Nisan, Shimon Schocken.

Luckily for us, NES doesn't have an Operating System. That means that the Application layer (Gamezzz) communicates with hardware directly, still using machine language.

The symplified version of this layered architecture looks like this:

<div style="text-align:center"><img src="./images/ch2/image_2_nes_emul_arch.png" width="30%"/></div>

As you can see, the machine language is the interface between our emulator and  NES games.

In the coming emulator, we would need to implement NES Computer Architecture, Arithmetic Logic Unit, and Memory. By using high-level language, we don't need to worry about simulating boolean arithmetic and sequential logic. We would rely on existing Rust features and language constructs.

## NES Platform Main Components

<div style="text-align:center"><img src="./images/ch2/image_3_nes_components.png" width="50%"/></div>

The significantly simplified schema of main NES hardware components: 

 * Central Processing Unit (**CPU**) is a modified version of [6502 chip](https://en.wikipedia.org/wiki/MOS_Technology_6502) - 2A03. As with any CPU, the goal of this module is to execute the main program instructions. 

* Picture Processing Unit (**PPU**) - was based on chip 2C02 made by the same company that made CPU - Ricoh. This module's primary goal is to draw the current state of a game on a TV Screen. 

* Both CPU and PPU have access to their 2 KiB (2048 bytes) banks of Random Access Memory (**RAM**)

* Audio Processing Unit (**APU**) - the module is a part of 2A03 chip and is responsible for generating specific five-channel based sounds, that made NES chiptunes so recognizable.

* Cartridges - were an essential part of the platform, mainly because the console didn't have an operating system. Each cartridge carried at least two large ROM chips - Character ROM (CHR ROM) and Program ROM (PRG ROM). The former stored video graphics data of a game, the latter stored CPU instructions - code of a game. 
(in reality, when a cartridge is inserted into the slot CHR Rom is connected directly to PPU, while PRG Rom is connected directly to CPU)
The later version of cartridges carried additional hardware (ROM and RAM) accessible through so-called mappers. That explains why later games had provided significantly better gameplay and visuals despite running on the same console hardware. 

<div style="text-align:center"><img src="./images/ch2/image_4_cartridge.png" width="50%"/></div>

* Gamepads - have a distinct goal to read inputs from a gamer and make it available for game logic. As we will learn later, the fact that the gamepad for the 8-bit platform has only eight buttons is not a coincidence. 

What's interesting is that CPU, PPU, and APU are independent of each other. This fact makes NES a distributed system in which separate components have to coordinate to generate one seamless gaming experience.

We can use the schema of the main NES components as an implementation plan for our emulator. 

<div style="text-align:center"><img src="./images/ch2/image_6_impl_plan.png" width="80%"/></div>

We have to build a simulation of all of these modules. The goal is to have something playable as soon as possible. Using iterative approach, we would incrementally add features to achieve the goal. 

I'm roughly estimating the effort required for each component in percentages. PPU is the hardest one, and the BUS is the easiest.

Writing a perfect emulator is a never-ending quest. But this quest has a start. And we will start by emulating the CPU.

<div style="text-align:center"><img src="./images/ch2/image_5_motherboard.png" width="80%"/></div>
