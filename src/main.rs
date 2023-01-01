mod cpu;
mod emulator;
mod input;
mod memory;
mod sdl_wrapper;
mod video;

extern crate sdl2;

use sdl2::event::Event;

use std::time::Duration;

pub fn main() {
    //Rewrite this into some kind of emulator config
    let rom_path = std::env::args().nth(1).expect("no rom path given");

    let print_cpu = match std::env::args().nth(2).unwrap_or_default().as_str() {
        "true" => true,
        _ => false,
    };

    let use_stepping = match std::env::args().nth(3).unwrap_or_default().as_str() {
        "true" => true,
        _ => false,
    };

    let mut sdl = sdl_wrapper::SdlWrapper::new();

    let config = emulator::Config::new(print_cpu, use_stepping, 0);
    let mut emulator = emulator::Emulator::new(config);
    emulator.load_rom(&rom_path.to_string());

    let mut input = input::Input::new();
    let mut canvas = sdl.get_window_canvas(
        "Gameboy Emulator",
        (video::SCREEN_WIDTH * video::PIXEL_SIZE) as u32,
        (video::SCREEN_HEIGHT * video::PIXEL_SIZE) as u32,
    );

    'running: loop {
        let events = sdl.get_events();
        for e in events {
            if let Event::Quit { .. } = e {
                break 'running;
            }
            input.consume_keys(e)
        }

        emulator.tick(&input);
        canvas.clear();
        if emulator.draw(&mut canvas) {
            canvas.present();
        }
        //::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 300));
    }
}
