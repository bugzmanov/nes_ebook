pub mod bus;
pub mod cartridge;
pub mod cpu;
pub mod opcodes;
pub mod trace;
pub mod ppu;
pub mod render;

use bus::Bus;
use cartridge::Rom;
use cpu::CPU;
// use trace::trace;
use render::frame::Frame;
use ppu::NesPPU;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate bitflags;

fn main() {
    // init sdl2
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("Tile viewer", (256.0 * 3.0) as u32, (240.0 * 3.0) as u32)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().present_vsync().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    canvas.set_scale(3.0, 3.0).unwrap();

    let creator = canvas.texture_creator();
    let mut texture = creator
        .create_texture_target(PixelFormatEnum::RGB24, 256, 240)
        .unwrap();

    //load the game
    let bytes: Vec<u8> = std::fs::read("pacman.nes").unwrap();
    let rom = Rom::new(&bytes).unwrap();

    let mut frame = Frame::new();

    // run the game cycle
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
