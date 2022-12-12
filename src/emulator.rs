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
        let result = fs::read(file_path).expect("file not found") ;
        self.memory.load(result);
    }

    pub fn draw(&mut self, canvas: &mut Canvas<Window>) {
        self.memory.draw(canvas);
    }

    pub fn reset(&mut self){
        self.memory.reset();
    }

    pub fn tick(&mut self, keys: &Input) {
        self.cpu.tick(&mut self.memory);
        self.memory.tick(self.cpu.get_clock_t());
    }
}
