# Rendering Static Screen

At this point the CPU and PPU are fully functional and working in coordination with each other. 
So if we load a game into our emulator, the game would be actually executing and most likely would run into demo mode.
 
The problem is that we can't see what's going on inside. Remember how we had intercepted the execution of the snake game to read game screen state? And then had it rendered via SDL2 canvas? We will have to do something similar here. It's just the data format a frame used by NES is slightly more complicated.

PPU has to deal with 2 categories of objects:

<div style="text-align:center"><img src="./images/ch6.4/image_8_bg_sprites_game.png" width="80%"/></div>

Both of those are constructed using CHR tiles, we've discussed in the last chapter. 
In fact, the same tile can be used both for background and for sprites. 

But NES uses different memory spaces to hold background and sprites. Also the set of possible transformations is different. 


## Rending Background

<!-- <div style="text-align:center"><img src="./images/ch6.4/image_1_pacman_bg.png" width="30%"/></div> -->

Three main memory sections are responsible for the state of a background:
- Pattern Table - 2 banks of tiles from CHR ROM on a cartridge
- Nametable - state of a screen stored in VRAM
- Palette table - the information about real colouring of pixels, stored in internal PPU memory

NES Screen background screen is composed of 960 tiles (a tile being 8x8 pixels: `256 / 8 * 240 / 8  = 960`) 
To hold that information NES allocate 960 bytes in VRAM in so called Nametable. Each byte in a Nametable holds an index of a tile to be used in current position. 


<div style="text-align:center"><img src="./images/ch6.4/image_2_nametable.png" width="100%"/></div>

> The pattern table consists of 2 banks of tiles and each bank holds 256 tiles. One byte in a name table can address only 256 elements within a single bank. 
> In addition, PPU relied on Control register that specified which bank should be used for background and sprites
> <div style="text-align:left"><img src="./images/ch6.4/image_3_control_register_highlight.png" width="50%"/></div>

In addition to 960 bytes for tiles a nametable holds 64 bytes that specify color palette, we will discuss later. For now, it's important to understand that a single frame is defined by 1024 bytes (960 + 64). The fact that PPU VRAM has 2048 bytes means that the PPU can simultaneously hold 2 nametables - state of 2 frames. 
2 additional nametables that exist in the address space of the PPU, must be either mapped to existing tables or to additional RAM space on a cartridge. 
More details: http://wiki.nesdev.com/w/index.php/Mirroring

The good news is that nametables are populated by CPU during program execution (using Addr and Data registers that we've implemented). So in order for us to get a screen state we just need to read it from an appropriate nametable.  

The algorithm to draw current background tiles:
1) Determine which nametable being used for the current screen (bit 0 and bit 1 or a Control register)
2) Determine which CHR ROM bank is used for background tiles
3) Read 960 bytes from the specified nametable and draw a 32x30 tile based screen by looking up each tile by index in the specified CHR ROM bank.

Lets add ```render``` function to render module:

```rust
pub mod frame;
pub mod palette;
 
use crate::ppu::NesPPU;
use frame::Frame;
 
pub fn render(ppu: &NesPPU, frame: &mut Frame) {
   let bank = ppu.ctrl.bknd_pattern_addr();
 
   for i in 0..0x03c0 { // just for now, lets use the first nametable
       let tile = ppu.vram[i] as u16;
       let tile_x = i % 32;
       let tile_y = i / 32;
       let tile = &ppu.chr_rom[(bank + tile * 16) as usize..=(bank + tile * 16 + 15) as usize];
 
       for y in 0..=7 {
           let mut upper = tile[y];
           let mut lower = tile[y + 8];
 
           for x in (0..=7).rev() {
               let value = (1 & upper) << 1 | (1 & lower);
               upper = upper >> 1;
               lower = lower >> 1;
               let rgb = match value {
                   0 => palette::SYSTEM_PALLETE[0x01],
                   1 => palette::SYSTEM_PALLETE[0x23],
                   2 => palette::SYSTEM_PALLETE[0x27],
                   3 => palette::SYSTEM_PALLETE[0x30],
                   _ => panic!("can't be"),
               };
               frame.set_pixel(tile_x*8 + x, tile_y*8 + y, rgb)
           }
       }
   }
}

```

Note: that we are still using random colors from a system palette for color index.

One last point: we need to define when we should intercept the program execution and read the screen state. 
On the real console, PPU is drawing one pixel each PPU clock cycle. However we don't need to do that, we can wait for the whole frame to be ready and draw it in one go. 

In reality, PPU was actively drawing screen state on a TV screen during 0 - 240 scanlines, then during scanlines 241 - 262 the CPU was updating state of PPU for the next frame, then the cycle repeated.

So it looks like the easiest would be to intercept PPU before it moves to from scanline 262 to 0.

First lets add callback to the bus, that will be triggered every time PPU jumps from 262 to 0 scanline:

```rust
ub struct Bus<'call> {
   cpu_vram: [u8; 2048],
   prg_rom: Vec<u8>,
   ppu: NesPPU,
 
   cycles: usize,
   gameloop_callback: Box<dyn FnMut(&NesPPU) + 'call>,
 
}
 
impl<'a> Bus<'a> {
   pub fn new<'call, F>(rom: Rom, gameloop_callback: F) -> Bus<'call>
   where
       F: FnMut(&NesPPU) + 'call,
   {
       let ppu = NesPPU::new(rom.chr_rom, rom.screen_mirroring);
 
       Bus {
           cpu_vram: [0; 2048],
           prg_rom: rom.prg_rom,
           ppu: ppu,
           cycles: 0,
           gameloop_callback: Box::from(gameloop_callback),
       }
   }
}
```

Then lets tweak ```tick``` function:

```rust
impl<'a> Bus<'a> {
//..
   pub fn tick(&mut self, cycles: u8) {
       self.cycles += cycles as usize;
       let new_frame = self.ppu.tick(cycles * 3);
       if new_frame {
           (self.gameloop_callback)(&self.ppu);
       }
   }
}
```

Then our main game loop would become:

```rust
fn main() {
   // init sdl2…
 
   //load the game
   let bytes: Vec<u8> = std::fs::read("game.nes").unwrap();
   let rom = Rom::new(&bytes).unwrap();
 
   let mut frame = Frame::new();
 
   // the game cycle
   let bus = Bus::new(rom, move |ppu: &NesPPU| {
       render::render(ppu, &mut frame);
       texture.update(None, &frame.data, 256 * 3).unwrap();
 
       canvas.copy(&texture, None, None).unwrap();
 
       canvas.present();
       for event in event_pump.poll_iter() {
           match event {
             Event::Quit { .. }
             | Event::KeyDown {
                 keycode: Some(Keycode::Escape),
                 ..
             } => std::process::exit(0),
             _ => { /* do nothing */ }
           }
        }
   });
 
   let mut cpu = CPU::new(bus);
 
   cpu.reset();
   cpu.run();
}
```

It's working!


<div style="text-align:center"><img src="./images/ch6.4/image_4_pacman_result.png" width="30%"/></div>

 Beautiful. Now let's fix the colors.

## Working with Colors

NES Console could generate 52 different colors on a TV screen. Those colors constitute the hardwired System Palette of the console. 

However a single screen could use only 25 colors simultaneously: 13 background colors and 12 for sprites. 

NES had internal memory RAM to store palette settings for a screen, in so called palette tables. 
The space was divided in 8 palettes: 4 for background and 4 for sprites. Each palette contains 3 colors. 
Remember that a pixel in a tile was coded using 2 bits - that's 4 possible values. 

> **0b00** for *background* tile means using Universal background color (at **0x3F00**). 
>
> For *sprites* - **0b00** means that the pixel is transparent


<div style="text-align:center"><img src="./images/ch6.4/image_5_palette_table.png" width="100%"/></div>

A single tile can be drawn using only one palette from the palette table. 
For background tiles, the last 64 bytes of each nametable are reserved for assigning a specific palette to a part of a background. This section is called an attribute table.

A byte in an attribute table controls palettes for 4 meta-tiles. (a meta tile is a space composed of 4x4 tiles)
A byte is composed of four 2bits blocks. And each block is assigning a background palette for four neighboring tiles. 

<div style="text-align:center"><img src="./images/ch6.4/image_6_attribute_table.png" width="70%"/></div>

First let's extract palette for a tile specified by its row and column position on a screen:

```rust
fn bg_pallette(ppu: &NesPPU, tile_column: usize, tile_row : usize) -> [u8;4] {
   let attr_table_idx = tile_row / 4 * 8 +  tile_column / 4;
   let attr_byte = ppu.vram[0x3c0 + attr_table_idx];  // note: still using hardcoded first nametable
 
   let pallet_idx = match (tile_column %4 / 2, tile_row % 4 / 2) {
       (0,0) => attr_byte & 0b11,
       (1,0) => (attr_byte >> 2) & 0b11,
       (0,1) => (attr_byte >> 4) & 0b11,
       (1,1) => (attr_byte >> 6) & 0b11,
       (_,_) => panic!("should not happen"),
   };
 
   let pallete_start: usize = 1 + (pallet_idx as usize)*4;
   [ppu.palette_table[0], ppu.palette_table[pallete_start], ppu.palette_table[pallete_start+1], ppu.palette_table[pallete_start+2]]
}
```

And just rewire our color lookup in `render` function:

```rust
pub fn render(ppu: &NesPPU, frame: &mut Frame) {
   let bank = ppu.ctrl.bknd_pattern_addr();
 
   for i in 0..0x3c0 {
       let tile = ppu.vram[i] as u16;
       let tile_column = i % 32;
       let tile_row = i / 32;
       let tile = &ppu.chr_rom[(bank + tile * 16) as usize..=(bank + tile * 16 + 15) as usize];
       let palette = bg_pallette(ppu, tile_column, tile_row);
 
       for y in 0..=7 {
           let mut upper = tile[y];
           let mut lower = tile[y + 8];
 
           for x in (0..=7).rev() {
               let value = (1 & lower) << 1 | (1 & upper);
               upper = upper >> 1;
               lower = lower >> 1;
               let rgb = match value {
                   0 => palette::SYSTEM_PALLETE[ppu.palette_table[0] as usize],
                   1 => palette::SYSTEM_PALLETE[palette[1] as usize],
                   2 => palette::SYSTEM_PALLETE[palette[2] as usize],
                   3 => palette::SYSTEM_PALLETE[palette[3] as usize],
                   _ => panic!("can't be"),
               };
               frame.set_pixel(tile_column * 8 + x, tile_row * 8 + y, rgb)
           }
       }
   }
}
```

That's it.

## Rendering sprites.

Rendering sprites is somewhat similar, yet a bit easier. 
NES had an internal RAM for storing states of all sprites in the frame, so called Object Attribute Memory (OAM).

It had 256 bytes of RAM, and reserved 4 bytes for each. This gives an option of having 64 tiles on a screen simultaneously (but keep in mind that a single object of a screen usually consists of 4 tiles).

In comparison to background tiles, a sprite tile can be shown anywhere in 256x240 screen, thus each OAM record has 2 bytes reserved for X and Y coordinates, one byte is used to select a tile pattern from the pattern table. And the last one specifies how the object would look like

NES Dev Wiki provides a pretty solid specification of each byte in the OAM record: http://wiki.nesdev.com/w/index.php/PPU_OAM

So rendering all sprites we just need to scan through oam_data space and parse out every 4 bytes into sprite:

```rust

pub fn render(ppu: &NesPPU, frame: &mut Frame) {

//.. draw background
//draw sprites
   for i in (0..ppu.oam_data.len()).step_by(4).rev() {
       let tile_idx = ppu.oam_data[i + 1] as u16;
       let tile_x = ppu.oam_data[i + 3] as usize;
       let tile_y = ppu.oam_data[i] as usize;
 
       let flip_vertical = if ppu.oam_data[i + 2] >> 7 & 1 == 1 {
           true
       } else {
           false
       };
       let flip_horizontal = if ppu.oam_data[i + 2] >> 6 & 1 == 1 {
           true
       } else {
           false
       };
       let pallette_idx = ppu.oam_data[i + 2] & 0b11;
       let sprite_palette = sprite_palette(ppu, pallette_idx);
      
       let bank: u16 = ppu.ctrl.sprt_pattern_addr();
 
       let tile = &ppu.chr_rom[(bank + tile_idx * 16) as usize..=(bank + tile_idx * 16 + 15) as usize];
 
 
       for y in 0..=7 {
           let mut upper = tile[y];
           let mut lower = tile[y + 8];
           'ololo: for x in (0..=7).rev() {
               let value = (1 & lower) << 1 | (1 & upper);
               upper = upper >> 1;
               lower = lower >> 1;
               let rgb = match value {
                   0 => continue 'ololo, // skip coloring the pixel
                   1 => palette::SYSTEM_PALLETE[sprite_palette[1] as usize],
                   2 => palette::SYSTEM_PALLETE[sprite_palette[2] as usize],
                   3 => palette::SYSTEM_PALLETE[sprite_palette[3] as usize],
                   _ => panic!("can't be"),
               };
               match (flip_horizontal, flip_vertical) {
                   (false, false) => frame.set_pixel(tile_x + x, tile_y + y, rgb),
                   (true, false) => frame.set_pixel(tile_x + 7 - x, tile_y + y, rgb),
                   (false, true) => frame.set_pixel(tile_x + x, tile_y + 7 - y, rgb),
                   (true, true) => frame.set_pixel(tile_x + 7 - x, tile_y + 7 - y, rgb),
               }
           }
       }
   }
```

And the sprite palette lookup is very easy:

```rust 
fn sprite_palette(ppu: &NesPPU, pallete_idx: u8) -> [u8; 4] {
    let start = 0x11 + (pallete_idx * 4) as usize;
    [
        0,
        ppu.palette_table[start],
        ppu.palette_table[start + 1],
        ppu.palette_table[start + 2],
    ]
}
```

<div style="text-align:center"><img src="./images/ch6.4/image_7_pacman_chrs.png" width="30%"/></div>


If you don't see characters in pacman specifically, most likely this means there is some issues in the way you've implemented memory mapping for OAM DMA register (0x4014) 
