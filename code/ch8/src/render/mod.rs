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

fn bg_pallette2(ppu: &NesPPU, attribute_table: [u8; 64], tile_column: usize, tile_row: usize) -> [u8; 4] {
    let attr_table_idx = tile_row / 4 * 8 + tile_column / 4;
    let attr_byte = attribute_table[attr_table_idx]; 

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

fn bg_tile(ppu: &NesPPU, tile_idx: u16) -> &[u8] {
    let bank = ppu.ctrl.bknd_pattern_addr();
    &ppu.chr_rom[(bank + tile_idx * 16) as usize..=(bank + tile_idx * 16 + 15) as usize]
}

fn render_name_table(ppu: &NesPPU, frame: &mut Frame, name_table: [u8; 960], attribute_table: [u8; 64], 
    view_port: Rect, position_x: usize, position_y: usize) {

    for i in 0..0x3c0 {
        let tile_column = i % 32;
        let tile_row = i / 32;
        let tile = bg_tile(ppu, i as u16);
        let palette = bg_pallette2(ppu, attribute_table, tile_column, tile_row);

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
                    frame.set_pixel(position_x + pixel_x, position_y + pixel_y, rgb);
                }
            }
        }
    }
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


                if ppu.ctrl.nametable_addr() == 0x2000 && pixel_x >= scroll_x {
                    frame.set_pixel(pixel_x - scroll_x, pixel_y, rgb);
                }

                if ppu.ctrl.nametable_addr() == 0x2400 && pixel_x  < scroll_x {
                    frame.set_pixel(256 - (scroll_x - pixel_x), pixel_y, rgb);
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

                if ppu.ctrl.nametable_addr() == 0x2000 && (pixel_x - 256) < scroll_x {
                    frame.set_pixel(pixel_x - scroll_x, pixel_y, rgb);
                }
                if ppu.ctrl.nametable_addr() == 0x2400 && pixel_x - 256  >= scroll_x {
                    frame.set_pixel(pixel_x - 255 - scroll_x, pixel_y, rgb);
                }
            }
        }
    }

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

        let tile =
            &ppu.chr_rom[(bank + tile_idx * 16) as usize..=(bank + tile_idx * 16 + 15) as usize];

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
                    (false, false) => {
                        frame.set_pixel(tile_x + x , tile_y + y, rgb);
                        // frame.set_pixel(tile_x + x, tile_y + y +250, rgb);
                    },
                    (true, false) => {
                        frame.set_pixel(tile_x + 7 - x , tile_y + y , rgb);
                        // frame.set_pixel(tile_x + 7 - x , tile_y + y + 250, rgb);
                    }
                    (false, true) => {
                        frame.set_pixel(tile_x + x  , tile_y + 7 - y, rgb);
                        // frame.set_pixel(tile_x + x, tile_y + 7 - y + 250, rgb);
                    }
                    (true, true) => {
                        frame.set_pixel(tile_x + 7 - x , tile_y + 7 - y , rgb);
                        // frame.set_pixel(tile_x + 7 - x, tile_y + 7 - y+250, rgb);
                    }
                }
            }
        }
    }
}
