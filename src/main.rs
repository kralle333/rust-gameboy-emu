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
    let mut sdl = sdl_wrapper::SdlWrapper::new();

    let mut emulator = emulator::Emulator::new();
    let rom_path = std::env::args().nth(1).expect("no rom path given");
    emulator.load_game(rom_path.to_string());

    let mut input = input::Input::new();
    let mut canvas = sdl.get_window_canvas(
        "Gameboy Emulator",
        video::SCREEN_WIDTH * video::PIXEL_SIZE,
        video::SCREEN_HEIGHT * video::PIXEL_SIZE,
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
        emulator.draw(&mut canvas);
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 300));
    }
}
