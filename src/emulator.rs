use std::fs;

use sdl2::event::Event;
use sdl2::keyboard;
use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::cpu::Cpu;
use crate::input::{Button, Input};
use crate::memory::Memory;

#[derive(PartialEq)]
enum DebugMode {
    None,
    Stepping,
    Breakpoint(u16),
}

pub struct Emulator {
    cpu: Cpu,
    memory: Memory,
    config: Config,
    debug_mode: DebugMode,
    loaded_rom: String,
    step_one: bool,
}

#[derive(Default)]
pub(crate) struct Config {
    print_cpu: bool,
    stepping_enabled: bool,
    breakpoint: u16,
}

impl Config {
    pub fn new(print_cpu: bool, stepping_enabled: bool, breakpoint: u16) -> Self {
        Self {
            print_cpu,
            stepping_enabled,
            breakpoint,
        }
    }
}
impl Emulator {
    pub(crate) fn new(config: Config) -> Emulator {
        let mode = Self::get_initial_debug_mode(&config);
        Emulator {
            cpu: Cpu::new(),
            memory: Memory::new(),
            config,
            debug_mode: mode,
            loaded_rom: "".to_string(),
            step_one: false,
        }
    }
    fn get_initial_debug_mode(config: &Config) -> DebugMode {
        if config.breakpoint != 0 {
            DebugMode::Breakpoint(config.breakpoint)
        } else if config.stepping_enabled {
            DebugMode::Stepping
        } else {
            DebugMode::None
        }
    }
    pub fn load_rom(&mut self, file_path: &String) {
        let result = fs::read(file_path).expect("file not found");
        self.memory.load(result);
        self.loaded_rom = file_path.to_string();
        println!("Loaded rom {file_path}");
    }

    fn reload_rom(&mut self) {
        let path = &self.loaded_rom.to_string();
        self.reset();
        self.load_rom(path);
    }

    pub fn draw(&mut self, canvas: &mut Canvas<Window>) -> bool {
        return self.memory.draw(canvas);
    }

    pub fn reset(&mut self) {
        self.memory.reset();
    }

    pub fn tick(&mut self, keys: &Input) {
        if keys.is_new_down(&Button::Reset) {
            self.reload_rom();
            return;
        }
        if keys.is_new_down(&Button::Step) {
            self.step_one = true;
        }
        if keys.is_new_down(&Button::ToggleStepping) {
            self.debug_mode = if self.debug_mode == DebugMode::Stepping {
                DebugMode::None
            } else {
                DebugMode::Stepping
            };
        }
        match self.debug_mode {
            DebugMode::None => {}
            DebugMode::Stepping => {
                if !self.step_one {
                    return;
                }
            }
            DebugMode::Breakpoint(addr) => {
                if self.cpu.PC() == addr {
                    self.debug_mode = DebugMode::Stepping;
                    self.config.print_cpu = true;
                    println!("Reached breakpoint {}, entering stepping mode", addr);
                    return;
                }
            }
        }
        self.cpu.tick(&mut self.memory);
        self.memory.tick(self.cpu.get_clock_t());
        self.step_one = false;
        if self.config.print_cpu {
            self.cpu.print();
        }
    }
}
