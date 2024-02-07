use std::fs;
use std::fs::OpenOptions;
use sdl2::rect::{Rect};
use sdl2::render::{Canvas, Texture};
use sdl2::video::Window;
use serde::Deserialize;

use crate::cartridge::Cartridge;
use crate::cpu::Cpu;
use crate::input::{Button, Input};
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
    config: RunConfig,
    debug_mode: DebugMode,
    loaded_rom: String,
    step_one: bool,
    draw_tiles: bool,
}

#[derive(Default, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RunConfig {
    pub(crate) path_to_rom: String,
    use_doctor: bool,
    breakpoint_at_pc: u16,
    breakpoint_at_instruction_count: u128,
    print_cpu: bool,
    use_stepping: bool,
}

impl RunConfig {
    pub(crate) fn validate(&self) {
        // panic if not valid
    }
}

impl Emulator {
    pub(crate) fn new(config: RunConfig) -> Emulator {
        let mode = Self::get_initial_debug_mode(&config);
        if config.use_doctor{
            OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true) // Truncate the file, removing existing content
                .open("blargg_log_instr.txt")
                .expect("cannot open file");
        }
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
    fn get_initial_debug_mode(config: &RunConfig) -> DebugMode {
        if config.breakpoint_at_pc != 0 {
            DebugMode::Breakpoint(config.breakpoint_at_pc)
        } else if config.use_stepping {
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
    pub fn draw_texture(&mut self, texture: &mut Texture) -> bool{
        return self.memory.draw_texture(texture);
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
        for i in 0..tilemap.len() {
            if i != 0 && (i % 32 == 0) {
                println!();
                x_offset = 0;
                y_offset += 8 * video::PIXEL_SIZE as i32;
            }
            let tile_index = tilemap[i] as usize;
            draw_tile(&bg_tiles[tile_index], x_offset as i32, y_offset as i32);
            x_offset += 8 * video::PIXEL_SIZE as i32;
            print!("{},", tile_index);
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

    fn tick_debug(&mut self) {
        if self.debug_mode == DebugMode::None {
            let mut should_step = false;
            if self.cpu.has_reached_operation_count(self.config.breakpoint_at_instruction_count) {
                should_step= true;
                println!("Stepping: reached breakpoint instruction count {}",self.config.breakpoint_at_instruction_count);
            } else if self.cpu.PC() == self.config.breakpoint_at_pc && self.config.breakpoint_at_pc != 0{
                should_step= true;
                println!("Stepping: reached breakpoint PC {}",self.config.breakpoint_at_pc);
            }
            if should_step{
                self.debug_mode = DebugMode::Stepping;
                self.config.print_cpu = true;
                if self.config.use_doctor{
                    self.cpu.write_buffered_doctor_lines();
                }
            }
        }
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
            return false;
        }
        if keys.is_new_down(&Button::Continue) && self.debug_mode == DebugMode::Stepping {
            self.debug_mode = DebugMode::Breakpoint(self.config.breakpoint_at_pc);
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

        true
    }

    pub fn get_last_clock_t(&self) -> u32 {
        self.cpu.get_clock_t()
    }

    pub fn tick(&mut self, keys: &Input) {
        if !self.check_debug_input(keys) {
            return;
        }
        self.tick_debug();
        if self.debug_mode == DebugMode::Stepping && !self.step_one{
            return;
        }

        self.cpu.tick(&mut self.memory);
        self.memory.tick(self.cpu.get_clock_t());
        self.step_one = false;
        if self.config.print_cpu {
            self.cpu.print();
        }
        if self.config.use_doctor{
            self.cpu.write_doctor();
        }
    }
}
