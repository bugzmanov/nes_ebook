# Emulating Picture Processing Unit

Picture Processing Unit is the hardest one to emulate because it deals with the most complicated aspect of gaming: rendering the state of the screen. The NES PPU has fair number of quirks. While emulating some of them is not necessarily required, others are crucially important to have a playable environment.
64KiB is not a hell of a lot of space, and NES platform designers tried to squeeze as much out of it as possible. Working with CHR ROM data means pretty much working with compressed data format; it requires a lot of bit arithmetic, uncompressing, and parsing.

 <div style="text-align:center"><img src="./images/ch6/image_1_ppu_failures.png" width="60%"/></div>


We will create the PPU emulator using four main steps:
* Emulating Registers and NMI Interruption
* Parsing and drawing tiles from CHR ROM
* Rendering PPU state:
    * Rendering background tiles
    * Rendering sprites
* Implementing the scroll

The first step is very similar to emulating the CPU.
After the third one it will be possible to play games with static screens:
- [Pac-Man](https://en.wikipedia.org/wiki/Pac-Man)
- [Donkey Kong](https://en.wikipedia.org/wiki/Donkey_Kong)
- [Balloon Fight](https://en.wikipedia.org/wiki/Balloon_Fight)

When we are done with the scroll, we could play platformers such as [Super Mario Bros](https://en.wikipedia.org/wiki/Super_Mario_Bros).

So let's start.
