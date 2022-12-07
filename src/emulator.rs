use std::fs;

use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::cpu::Cpu;
use crate::input::Input;
use crate::memory::Memory;

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
    pub fn load_game(&mut self, file_path: String) {
        match fs::read(file_path) {
            Ok(data) => {
                self.memory.load(data);
            }
            Err(e) => {
                panic!("ERROR {}", e);
            }
        }
    }

    pub fn draw(&mut self, canvas: &mut Canvas<Window>) {
        self.memory.draw(canvas);
    }

    pub fn tick(&mut self, keys: &Input) {
        self.cpu.tick(&mut self.memory)
    }
}
