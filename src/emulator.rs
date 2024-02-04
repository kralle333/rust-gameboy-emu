use std::fs;

use sdl2::rect::{Rect};
use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::cartridge::Cartridge;
use crate::cpu::Cpu;
use crate::input::{Button, Input};
use crate::input::Button::B;
use crate::memory::{Memory};
use crate::video;
use crate::video::GBColor;

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
    draw_tiles: bool,
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
            draw_tiles: false,
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
        println!("Loading rom {file_path}");
        let result = fs::read(file_path).expect("file not found");
        let cartridge = Cartridge::new(&result);
        println!(
            "Success: Rom Size {0}KB Ram {1}KB, Cartridge {2:?}",
            cartridge.rom_size / 1024,
            cartridge.ram_size / 1024,
            cartridge.cartidge_type
        );

        self.memory.load(result, &cartridge);
        self.loaded_rom = file_path.to_string();
    }

    fn reload_rom(&mut self) {
        let path = &self.loaded_rom.to_string();
        self.reset();
        self.load_rom(path);
    }

    pub fn draw(&mut self, canvas: &mut Canvas<Window>) -> bool {
        return self.memory.draw(canvas);
    }

    pub fn draw_debug(&mut self, canvas: &mut Canvas<Window>) -> bool {
        if !self.draw_tiles {
            return false;
        }

        let bg_tiles = self.memory.dump_tiles();
        let mut draw_tile = |tile: &[[GBColor; 8]; 8]/* Type */, offset_x, offset_y| {
            for x in 0..8 {
                for y in 0..8 {
                    let c = video::get_color(
                        &tile[y as usize][x as usize],
                        &video::ColorScheme::BlackWhite,
                    );
                    canvas.set_draw_color(c);
                    match canvas.fill_rect(Rect::new(
                        offset_x + x * video::PIXEL_SIZE as i32,
                        offset_y + y * video::PIXEL_SIZE as i32,
                        video::PIXEL_SIZE as u32,
                        video::PIXEL_SIZE as u32,
                    )) {
                        Ok(_) => {}
                        Err(err) => panic!("{err}"),
                    }
                }
            }
        };

        let tilemap = self.memory.debug_get_background_tilemap();

        let mut x_offset = 0;
        let mut y_offset = 0;
        for i in 0..tilemap.len(){
            if i != 0 && (i % 32 == 0) {
                println!();
                x_offset = 0;
                y_offset += 8 * video::PIXEL_SIZE as i32;
            }
            let tile_index = tilemap[i] as usize;
            draw_tile(&bg_tiles[tile_index],x_offset as i32 ,y_offset as i32);
            x_offset += 8 * video::PIXEL_SIZE as i32;
            print!("{},",tile_index);
        }

        let mut x_offset = 0;
        let mut y_offset = 0;
        for tile in bg_tiles {
            draw_tile(tile, x_offset, y_offset);
            x_offset += 8 * video::PIXEL_SIZE as i32;
            if x_offset > 384 * 2 {
                x_offset = 0;
                y_offset += 8 * video::PIXEL_SIZE as i32;
            }
        }
        self.draw_tiles = false;
        true
    }

    pub fn reset(&mut self) {
        self.cpu.reset();
        self.memory.reset();
    }

    fn check_debug_input(&mut self, keys: &Input) -> bool {
        if keys.is_new_down(&Button::Reset) {
            self.reload_rom();
            return false;
        }
        if keys.is_new_down(&Button::DumpBgTiles) {
            //self.memory.dump_bg_tiles();
            self.draw_tiles = true;
            return false;
        }
        if keys.is_new_down(&Button::Step) {
            self.step_one = true;
        }
        if keys.is_new_down(&Button::Continue) && self.debug_mode == DebugMode::Stepping {
            self.debug_mode = DebugMode::Breakpoint(self.config.breakpoint);
            self.step_one = false;
        }
        if keys.is_down(&Button::ToggleStepping) {
            self.debug_mode = if self.debug_mode == DebugMode::Stepping {
                DebugMode::None
            } else {
                DebugMode::Stepping
            };
        }
        if keys.is_new_down(&Button::ToggleBackground) {
            self.memory.debug_toggle_background();
        }
        if keys.is_new_down(&Button::ToggleWindow) {
            self.memory.debug_toggle_window();
        }
        if keys.is_new_down(&Button::ToggleObjects) {
            self.memory.debug_toggle_objects();
        }


        match self.debug_mode {
            DebugMode::None => {}
            DebugMode::Stepping => {
                if !self.step_one {
                    return false;
                }
            }
            DebugMode::Breakpoint(addr) => {
                if self.cpu.PC() == addr {
                    self.debug_mode = DebugMode::Stepping;
                    self.config.print_cpu = true;
                    println!("Reached breakpoint {:#0x}, entering stepping mode", addr);
                    return false;
                }
            }
        }
        true
    }

    pub fn tick(&mut self, keys: &Input) {
        if !self.check_debug_input(keys) {
            return;
        }

        self.cpu.tick(&mut self.memory);
        self.memory.tick(self.cpu.get_clock_t());
        self.step_one = false;
        if self.config.print_cpu {
            self.cpu.print();
        }
    }
}
