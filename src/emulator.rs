use std::collections::HashMap;
use std::fs;

use sdl2::keyboard::Keycode;
use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::cpu::Cpu;
use crate::input::Input;
use crate::mem::Memory;

const ROMS_DIR: &str = "<YOUR ROM PATH HERE>";

pub struct Emulator {
    cpu: Cpu,
    memory: Memory,
}

impl Emulator {
    pub fn new() -> Emulator {
        Emulator {
            cpu: Cpu::new(),
            memory: Memory::new(),
        }
    }
    pub fn load_game(&mut self, file_path: &str) {
        let file_path = ROMS_DIR.to_owned() + file_path;
        match fs::read(file_path) {
            Ok(data) => {
                self.memory.load(data);
            }
            Err(e) => {
                panic!("ERROR {}", e);
            }
        }
    }

    pub fn draw(&self, _canvas: &mut Canvas<Window>) {
        todo!()
    }

    pub fn tick(&mut self, keys: &Input) {
        self.cpu.tick(&mut self.memory)
    }
}
