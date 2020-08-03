# Emulating Picture Processing Unit

PPU is the hardest one to emulate. Because it deals with the most complicated aspect of gaming - rendering the state of the screen, but also PPU has quite a bit of quirks. While emulating some of them is not necessarily required, others are crucially important to have a playable environment. 
64KiB is not a hell lot of space, and NES platform designers tried to squeeze out of it as much as possible. Working with CHR ROM data means pretty much working with compressed data format; it requires a lot of bit arithmetic, uncompressing, and parsing.

 <div style="text-align:center"><img src="./images/ch6/image_1_ppu_failures.png" width="60%"/></div>


We would create the PPU emulator can four main steps:
* Emulating Registers and NMI Interruption
* Parsing and drawing tiles from CHR ROM
* Rendering PPU state: 
    * Rendering background tiles 
    * Rendering sprites
* Implementing scroll
    * Horizontal
    * Vertical

The first step is the easiest one and the task close to emulating the CPU. 
After the third one it will be possible to play games with static screens:
* Donkey Kong
* PacMan
* Balloon Fight

After implementing scroll, we would be able to play platformers like Super Mario Bros. 

So let's start