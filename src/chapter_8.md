# PPU Scrolling

Scroll is one of the main mechanisms to simulate movement in space in NES games. It's an old idea of moving the viewport against static background to create illusion of movement through space.

<div style="text-align:center;"><img src="./images/ch8/image_1_scroll_basics.png" width="80%"/></div>

Scroll is implemented on PPU level and affects only rendering of background tiles (those that are stored in nametables). Sprites (OAM data) is not affected by this.  

PPU can keep two screens in memory simultaneously (remember one name table - 1 KiB, and PPU has 2 KiB of VRAM). This doesn't look like a lot, but this is enough to make the trick. During the scroll the viewport cycles through those two nametables, while the CPU is busy updating the part of the screen that's not yet visible, but will be soon. 
That also means that for the majority of time the gamer sees parts of both nametables rendered. 

Because this exhausts all available console resources, early games had only 2 options for scrolling: horizontal or vertical. Early games were settled on the type of scrolling for the whole game. 
Games that came later on had a mechanism to alternate scrolling between stages. And the most advanced games (like Zelda) provided the experience where a user can "move" in all 4 directions. 

<div style="text-align:center;"><img src="./images/ch8/image_2_scroll_mirroring.png" width="60%"/></div>


Initially, the scroll was tightly coupled with mirroring - mostly because of the way NES handled overflow of a viewport from one nametable to another on hardware level. 

For games like Super Mario Bros (Horizontal Scroll) or Ice Climber (Vertical Scroll) the mechanism is fully defined by:
- Mirroring type (defined on a cartridge ROM level)
- Base Nametable address (value in PPU Control register)
- Status of PPU Scroll Register (X and Y shift values of the viewport, in pixels)
- Content of Nametables

Remember, a background screen is defined by 960 tiles, each tile being 8x8 pixels, because PPU Scroll Register defines shifts in pixels, that means that on edges of the viewport we can see parts of a tile


<div style="text-align:center;"><img src="./images/ch8/image_3_scroll_controll.png" width="70%"/></div>


Updating PPU memory is relatively expensive and the CPU can do this only durgin 241 - 262 scanlines. Because of these constraints the CPU can update a relatively thin part (2x30 tiles wide area) of a screen per frame. 
If we render parts of the nametables that are not yet visible, we can see how the state of the world comes into existence a couple frames before entering the view port. 

<div style="text-align:center;"><img src="./images/ch8/image_4_scroll_demo.gif" width="50%"/></div>

2 last notes before jumping into implementation:
* The palette of a tile is defined by the nametable the tile belongs to. 
* In case of horizontal scrolling content of the base nametable always goes to the left part of the viewport (or top part in case of vertical scrolling)


<div style="text-align:center;"><img src="./images/ch8/image_5_scroll_caveats.png" width="80%"/></div>

Implementing scroll rendering is not hard but requires attention to multiple details. The most convenient mental model I could come up with is the following:
* For each frame we would scan through both nametables.
* For each nametable we would specify visible part of the nametable:

```rust
struct Rect {
   x1: usize,
   y1: usize,
   x2: usize,
   y2: usize,
}
 
impl Rect {
   fn new(x1: usize, y1: usize, x2: usize, y2: usize) -> Self {
       Rect {
           x1: x1,
           y1: y1,
           x2: x2,
           y2: y2,
       }
   }
}
```

* And shift transformation for each visible pixel - shift_x, shift_y

> For example,
> <div style="text-align:center;"><img src="./images/ch8/image_6_transform_example.png" width="30%"/></div>
>
> For nametable **0x2400**: the visible area would be defined as **(200, 0, 256, 240)** and the shift would be **(-200, 0)**<br/>
> For nametable **0x2000**: the visible area is **(0,0, 200, 240)** and the shift is **(56, 0)**

So, to draw a nametable we need to create a helper function:

```rust
fn render_name_table(ppu: &NesPPU, frame: &mut Frame, name_table: &[u8],
   view_port: Rect, shift_x: isize, shift_y: isize) {
   let bank = ppu.ctrl.bknd_pattern_addr();
 
   let attribute_table = &name_table[0x3c0.. 0x400];
 
   for i in 0..0x3c0 {
       let tile_column = i % 32;
       let tile_row = i / 32;
       let tile_idx = name_table[i] as u16;
       let tile = &ppu.chr_rom[(bank + tile_idx * 16) as usize..=(bank + tile_idx * 16 + 15) as usize];
       let palette = bg_pallette(ppu, attribute_table, tile_column, tile_row);
 
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
               let pixel_x = tile_column * 8 + x;
               let pixel_y = tile_row * 8 + y;
 
               if pixel_x >= view_port.x1 && pixel_x < view_port.x2 && pixel_y >= view_port.y1 && pixel_y < view_port.y2 {
                   frame.set_pixel((shift_x + pixel_x as isize) as usize, (shift_y + pixel_y as isize) as usize, rgb);
               }
           }
       }
   }
}
```


Then rendering background becomes relatively simple:

```rust


pub fn render(ppu: &NesPPU, frame: &mut Frame) {
   let scroll_x = (ppu.scroll.scroll_x) as usize;
   let scroll_y = (ppu.scroll.scroll_y) as usize;
 
   let (main_nametable, second_nametable) = match (&ppu.mirroring, ppu.ctrl.nametable_addr()) {
       (Mirroring::VERTICAL, 0x2000) | (Mirroring::VERTICAL, 0x2800) => {
           (&ppu.vram[0..0x400], &ppu.vram[0x400..0x800])
       }
       (Mirroring::VERTICAL, 0x2400) | (Mirroring::VERTICAL, 0x2C00) => {
           ( &ppu.vram[0x400..0x800], &ppu.vram[0..0x400])
       }
       (_,_) => {
           panic!("Not supported mirroring type {:?}", ppu.mirroring);
       }
   };
 
   render_name_table(ppu, frame,
       main_nametable,
       Rect::new(scroll_x, scroll_y, 256, 240 ),
       -(scroll_x as isize), -(scroll_y as isize)
   );

    render_name_table(ppu, frame,
        second_nametable,
        Rect::new(0, 0, scroll_x, 240),
        (256 - scroll_x) as isize, 0
    );
   
// … render sprites
}

```

Implementing vertical scroll is pretty similar, we would reuse the same `render_name_table` helper function without changes. Just need to figure out proper *addressing*, *shifts* and *view_port* parameters.

The fully defined code for scrolling can be found [here]

Support for scrolling means that now we can play platformers like Super Mario Bros and Ice Climber.

The final missing peace is APU. 
