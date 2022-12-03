mod cpu;
mod emulator;
mod mem;
mod video;
mod input;

extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::collections::HashMap;
use std::time::Duration;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let _audio_subsystem = sdl_context.audio().expect("Unable to init audio");

    let window = video_subsystem
        .window(
            "GameBoy Emulator",
            (video::SCREEN_WIDTH * video::PIXEL_SIZE as usize) as u32,
            (video::SCREEN_HEIGHT * video::PIXEL_SIZE as usize) as u32,
        )
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();

    let mut emulator = emulator::Emulator::new();

    emulator.load_game("<Your rom file here!>");

    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();

   

    let mut input = input::Input::new();
  
    'running: loop {
        // Start playback
        for event in event_pump.poll_iter() {
            if let Event::Quit{ .. } = event { 
                break 'running;
             }
             input.consume_keys(event);
             break;
        }
        emulator.tick(&input);
        canvas.clear();
        emulator.draw(&mut canvas);
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 300));
    }
}
