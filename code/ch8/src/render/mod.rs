pub mod frame;
pub mod palette;

use crate::ppu::NesPPU;
use frame::Frame;

fn bg_pallette(ppu: &NesPPU, tile_column: usize, tile_row: usize, attr_table: usize) -> [u8; 4] {
    let attr_table_idx = tile_row / 4 * 8 + tile_column / 4;
    let attr_byte = ppu.vram[attr_table + attr_table_idx]; // note: still using hardcoded first nametable

    let pallet_idx = match (tile_column % 4 / 2, tile_row % 4 / 2) {
        (0, 0) => attr_byte & 0b11,
        (1, 0) => (attr_byte >> 2) & 0b11,
        (0, 1) => (attr_byte >> 4) & 0b11,
        (1, 1) => (attr_byte >> 6) & 0b11,
        (_, _) => panic!("should not happen"),
    };

    let pallete_start: usize = 1 + (pallet_idx as usize) * 4;
    [
        ppu.palette_table[0],
        ppu.palette_table[pallete_start],
        ppu.palette_table[pallete_start + 1],
        ppu.palette_table[pallete_start + 2],
    ]
}

fn sprite_palette(ppu: &NesPPU, pallete_idx: u8) -> [u8; 4] {
    let start = 0x11 + (pallete_idx * 4) as usize;
    [
        0,
        ppu.palette_table[start],
        ppu.palette_table[start + 1],
        ppu.palette_table[start + 2],
    ]
}



pub fn render(ppu: &NesPPU, frame: &mut Frame) {
    let bank = ppu.ctrl.bknd_pattern_addr();
    let scroll_x = (ppu.scroll.scroll_x) as usize;
    let scroll_y = (ppu.scroll.scroll_y) as usize;
    // println!("{}  - {}" ,scroll_x, scroll_y);
    // println!("{:x}", ppu.ctrl.nametable_addr());

    for i in 0..0x3c0 {
        let tile = ppu.vram[i] as u16;
        let tile_column = i % 32;
        let tile_row = i / 32;
        let tile = &ppu.chr_rom[(bank + tile * 16) as usize..=(bank + tile * 16 + 15) as usize];
        let palette = bg_pallette(ppu, tile_column, tile_row, 0x3c0);

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

                let mut visible = false;

                if ppu.ctrl.nametable_addr() == 0x2000 && pixel_x >= scroll_x {
                    frame.set_pixel(123 + pixel_x - scroll_x, pixel_y + 250, rgb);
                    visible = true;
                }

                if ppu.ctrl.nametable_addr() == 0x2400 && pixel_x  < scroll_x {
                    frame.set_pixel(123 + 256 - (scroll_x - pixel_x), pixel_y + 250, rgb);
                    visible = true;
                }

                if visible {
                    frame.set_pixel(pixel_x, pixel_y, rgb);
                } else {
                    frame.set_pixel(pixel_x, pixel_y, (rgb.0.saturating_sub(50), rgb.1.saturating_sub(50), rgb.2.saturating_sub(50)));
                }


            }
        }
    }

 
    for i in 0x400..0x7C0 {
        let tile = ppu.vram[i] as u16;
        let tile_column = (i - 0x400) % 32;
        let tile_row = (i - 0x400) / 32;
        let tile = &ppu.chr_rom[(bank + tile * 16) as usize..=(bank + tile * 16 + 15) as usize];
        let palette = bg_pallette(ppu, tile_column, tile_row, 0x3c0 + 0x400);

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
                let pixel_x = 256 + tile_column * 8 + x;
                let pixel_y = tile_row * 8 + y;

                // if ppu.ctrl.nametable_addr() == 0x2400 && pixel_x >= scroll_x as usize {
                //     frame.set_pixel(pixel_x - 256 - scroll_x, pixel_y + 250, rgb);
                // }

                let mut visible = false;

                if ppu.ctrl.nametable_addr() == 0x2000 && (pixel_x - 256) < scroll_x {
                    visible = true;
                    frame.set_pixel(123 + pixel_x - scroll_x, pixel_y + 250, rgb);
                }
                if ppu.ctrl.nametable_addr() == 0x2400 && pixel_x - 256  >= scroll_x {
                    visible = true;
                    frame.set_pixel(123 + pixel_x - 256 - scroll_x, pixel_y + 250, rgb);
                }

                if visible {
                    frame.set_pixel(pixel_x, pixel_y, rgb);
                } else {
                    frame.set_pixel(pixel_x, pixel_y, (rgb.0.saturating_sub(50), rgb.1.saturating_sub(50), rgb.2.saturating_sub(50)));
                }


            }
        }
    }
    for y in 0 .. 240 {
        frame.set_pixel(256, y, (255, 0, 0));
    }


    if ppu.ctrl.nametable_addr() == 0x2000 {


        for x in scroll_x .. (scroll_x + 256) {
            

            frame.set_pixel(x as usize, 0, (0, 0, 0));
            frame.set_pixel(x as usize, 240, (0, 0, 0));
        }
    
        for y in 0 .. 240 {
            frame.set_pixel(scroll_x as usize, y, (0, 0, 0));
            frame.set_pixel(scroll_x as usize + 256, y, (0, 0, 0));
        }
    } else {
        for x in (256 + scroll_x) .. 512 {
            frame.set_pixel(x as usize, 0, (0, 0, 0));
            frame.set_pixel(x as usize, 240, (0, 0, 0));
        }
    
        for y in 0 .. 240 {
            frame.set_pixel(256 + scroll_x as usize, y, (0, 0, 0));
            // frame.set_pixel(256 + scroll_x as usize + 256, y, (0, 0, 0));
        }
    
        for x in 0 .. scroll_x {
            frame.set_pixel(x as usize, 0, (0, 0, 0));
            frame.set_pixel(x as usize, 240, (0, 0, 0));
        }

        for y in 0 .. 240 {
            frame.set_pixel(scroll_x as usize, y, (0, 0, 0));
            // frame.set_pixel(256 + scroll_x as usize + 256, y, (0, 0, 0));
        }



    }



    // for i in (0..ppu.oam_data.len()).step_by(4).rev() {
    //     let tile_idx = ppu.oam_data[i + 1] as u16;
    //     let tile_x = ppu.oam_data[i + 3] as usize;
    //     let tile_y = ppu.oam_data[i] as usize;

    //     let flip_vertical = if ppu.oam_data[i + 2] >> 7 & 1 == 1 {
    //         true
    //     } else {
    //         false
    //     };
    //     let flip_horizontal = if ppu.oam_data[i + 2] >> 6 & 1 == 1 {
    //         true
    //     } else {
    //         false
    //     };
    //     let pallette_idx = ppu.oam_data[i + 2] & 0b11;
    //     let sprite_palette = sprite_palette(ppu, pallette_idx);
    //     let bank: u16 = ppu.ctrl.sprt_pattern_addr();

    //     let tile =
    //         &ppu.chr_rom[(bank + tile_idx * 16) as usize..=(bank + tile_idx * 16 + 15) as usize];

    //     for y in 0..=7 {
    //         let mut upper = tile[y];
    //         let mut lower = tile[y + 8];
    //         'ololo: for x in (0..=7).rev() {
    //             let value = (1 & lower) << 1 | (1 & upper);
    //             upper = upper >> 1;
    //             lower = lower >> 1;
    //             let rgb = match value {
    //                 0 => continue 'ololo, // skip coloring the pixel
    //                 1 => palette::SYSTEM_PALLETE[sprite_palette[1] as usize],
    //                 2 => palette::SYSTEM_PALLETE[sprite_palette[2] as usize],
    //                 3 => palette::SYSTEM_PALLETE[sprite_palette[3] as usize],
    //                 _ => panic!("can't be"),
    //             };
    //             match (flip_horizontal, flip_vertical) {
    //                 (false, false) => frame.set_pixel(tile_x + x, tile_y + y, rgb),
    //                 (true, false) => frame.set_pixel(tile_x + 7 - x, tile_y + y, rgb),
    //                 (false, true) => frame.set_pixel(tile_x + x, tile_y + 7 - y, rgb),
    //                 (true, true) => frame.set_pixel(tile_x + 7 - x, tile_y + 7 - y, rgb),
    //             }
    //         }
    //     }
    // }
}
