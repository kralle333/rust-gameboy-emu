mod cartridge;
mod cpu;
mod emulator;
mod input;
mod memory;
mod sdl_wrapper;
mod video;

extern crate sdl2;
extern crate serde;
extern crate serde_json;

use crate::emulator::RunConfig;

use std::path::Path;
use sdl2::{event::Event, pixels::Color};

use std::fs::read_to_string;

pub const FRAME_LENGTH: u32 = 69905;

fn arg_to_bool(arg: &str) -> bool {
    match arg {
        "true" => true,
        _ => false,
    }
}

pub fn main() {
    let first_argument = std::env::args().nth(1).expect("missing first argument");
    let path_to_arg = Path::new(&first_argument);
    if !path_to_arg.exists() {
        panic!("unknown file: {}", first_argument);
    }
    let (path_to_rom, config_to_use) = match first_argument.as_str() {
        _ if first_argument.ends_with(".gb") => {
            let (p, mut r) = (first_argument, RunConfig::default());
            if let Some(x) = std::env::args().nth(2) {
                r.print_cpu = arg_to_bool(&x);
            }
            (p, r)
        }
        _ if first_argument.ends_with(".json") => {
            let conf: RunConfig = serde_json::from_str(&read_to_string(path_to_arg).unwrap()).unwrap();
            let path_to_rom = conf.path_to_rom.to_string();
            (path_to_rom, conf)
        }
        _ => { panic!("unknown arg given") }
    };

    config_to_use.validate();

    let mut sdl = sdl_wrapper::SdlWrapper::new();
    let mut emulator = emulator::Emulator::new(config_to_use);
    emulator.load_rom(&path_to_rom);

    // let mut debug_canvas = sdl.get_window_canvas("tiles", 384 * 2, 500);
    //
    // debug_canvas.clear();
    // debug_canvas.present();

    let mut input = input::Input::new();
    let mut canvas = sdl.get_window_canvas(
        "Gameboy Emulator",
        (video::SCREEN_WIDTH * video::PIXEL_SIZE) as u32,
        (video::SCREEN_HEIGHT * video::PIXEL_SIZE) as u32,
    );


    let mut clock_t: u32 = 0;
    'running: loop {
        let events = sdl.get_events();
        for e in events {
            if let Event::Quit { .. } = e {
                break 'running;
            }
            input.consume_keys(e)
        }
        let target = clock_t + FRAME_LENGTH;
        while clock_t < target {
            emulator.tick(&input);
            clock_t += emulator.get_last_clock_t() as u32;
        }
        clock_t %= FRAME_LENGTH;
        canvas.set_draw_color(Color::BLACK);
        canvas.clear();
        if emulator.draw(&mut canvas) {
            canvas.present();
        }
        // debug_canvas.set_draw_color(Color::BLACK);
        // debug_canvas.clear();
        // if emulator.draw_debug(&mut debug_canvas) {
        //     debug_canvas.present();
        // }
        //::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 100000));
    }
}
