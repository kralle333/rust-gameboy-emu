mod gpu;
mod gpu_test;
mod mem_test;
mod mmu;
mod rom;
mod sound;

use std::{fs::File, io::Write, ops::Shl};

use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::{
    cartridge::Cartridge,
    video::{SCREEN_HEIGHT, SCREEN_WIDTH, self},
};

use self::{gpu::{Gpu}, mmu::Mmu, rom::Rom, sound::Sound};

const DIVIDER_ADD: i16 = 16384;

pub trait MemoryType {
    fn read_byte(&self, addr: u16) -> u8;
    fn write_byte(&mut self, addr: u16, val: u8);
    fn read_word(&self, addr: u16) -> u16 {
        let lsn = self.read_byte(addr) as u16;
        let msn = (self.read_byte(addr + 1) as u16).shl(8);
        msn | lsn
    }
    fn write_word(&mut self, addr: u16, val: u16) {
        let lsn = val & 0xFF;
        let msn = (val & 0xFF00) >> 8;
        self.write_byte(addr, lsn as u8);
        self.write_byte(addr + 1, msn as u8);
    }
}

struct DivRegister {
    val: u8,
    added_this_second: u8,
    time_since_last_check: u16,
}

pub struct Memory {
    rom: Rom,
    ram: Mmu,
    gpu: Gpu,
    snd: Sound,
    interupt_enable: u8,
    interupt_flag: u8,

    //special registers
    joypad: u8,                 // FF00
    serial_transfer_data: u8,   //FF01
    serial_transer_control: u8, //FF02
    div_register: DivRegister,  //FF04
    timer_counter: u8,          //FF05
    timer_modulo: u8,           //FF06
    timer_control: u8,          //FF07
}

impl MemoryType for Memory {
    fn read_byte(&self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x7fff => self.rom.read_byte(addr),
            0x8000..=0x9fff => self.gpu.read_byte(addr),
            0xa000..=0xfdff => self.rom.read_byte(addr),
            0xfe00..=0xfe9f => self.gpu.read_byte(addr),
            0xfea0..=0xfeff => {
                println!("Reading from empty but unusable for I/O");
                0
            }
            0xff00 => self.joypad,
            0xff01 => self.serial_transfer_data,
            0xff02 => self.serial_transer_control,
            0xff04 => self.div_register.val,
            0xff05 => self.timer_counter,
            0xff06 => self.timer_modulo,
            0xff07 => self.timer_control,
            0xff0f => self.interupt_flag,
            0xff10..=0xff3f => self.snd.read_byte(addr),
            0xff40..=0xff4b => self.gpu.read_byte(addr),
            0xff80..=0xfffe => self.rom.read_byte(addr),
            0xff4c..=0xff7f => {
                println!("Reading from empty but unusable for I/O");
                0
            }
            0xffff => self.interupt_enable,
            _ => {
                println!("Reading from invalid address {}", addr);
                0
            }
        }
    }

    fn write_byte(&mut self, addr: u16, val: u8) {
        match addr {
            0x0000..=0x7fff => self.rom.write_byte(addr, val),
            0x8000..=0x9fff => self.gpu.write_byte(addr, val),
            0xa000..=0xfdff => self.rom.write_byte(addr, val),
            0xfe00..=0xfe9f => self.gpu.write_byte(addr, val),
            0xfea0..=0xfeff => println!("Writing to empty but unusable for I/O"),
            0xff00 => self.joypad = val,
            0xff01 => self.serial_transfer_data = val,
            0xff02 => self.serial_transer_control = val,
            0xff04 => self.div_register.val = 0,
            0xff05 => self.timer_counter = val,
            0xff06 => self.timer_modulo = val,
            0xff07 => self.timer_control = val,
            0xff0f => self.interupt_flag = val,
            0xff10..=0xff3f => self.snd.write_byte(addr, val),
            0xff40..=0xff4b => self.gpu.write_byte(addr, val),
            0xff4c..=0xff7f => println!("Writing to empty but unusable for I/O"),
            0xff80..=0xfffe => self.rom.write_byte(addr, val),
            0xffff => self.interupt_enable = val,
            _ => println!("unused {}", addr),
        }
    }
}

impl Memory {
    pub fn new() -> Memory {
        let mut mem = Memory {
            rom: Rom::new(),
            ram: Mmu::new(),
            gpu: Gpu::new(),
            snd: Sound::new(),
            interupt_enable: 0,
            interupt_flag: 0,
            joypad: 0,
            serial_transfer_data: 0,
            serial_transer_control: 0,
            div_register: DivRegister {
                val: 0,
                added_this_second: 0,
                time_since_last_check: 0,
            },
            timer_counter: 0,
            timer_modulo: 0,
            timer_control: 0,
        };
        mem.reset();
        mem
    }

    pub fn load(&mut self, data: Vec<u8>, cartridge_info: &Cartridge) {
        self.rom.load(&data, cartridge_info);
    }
    pub fn reset(&mut self) {
        self.write_byte(0xFF00, 0x0F); //0x0F no buttons pressed
        self.write_byte(0xFF05, 0x00); //TIMA
        self.write_byte(0xFF06, 0x00); //TMA
        self.write_byte(0xFF07, 0x00); //TAC
        self.write_byte(0xFF10, 0x80); //NR10
        self.write_byte(0xFF11, 0xBF); //NR11
        self.write_byte(0xFF12, 0xF3); //NR12
        self.write_byte(0xFF14, 0xBF); //NR14
        self.write_byte(0xFF16, 0x3F); //NR21
        self.write_byte(0xFF17, 0x00); //NR22
        self.write_byte(0xFF19, 0xBF); //NR24
        self.write_byte(0xFF1A, 0x7F); //NR30
        self.write_byte(0xFF1B, 0xFF); //NR31
        self.write_byte(0xFF1C, 0x9F); //NR32
        self.write_byte(0xFF1E, 0xBF); //NR33
        self.write_byte(0xFF20, 0xFF); //NR41
        self.write_byte(0xFF21, 0x00); //NR42
        self.write_byte(0xFF22, 0x00); //NR43
        self.write_byte(0xFF23, 0xBF); //NR30
        self.write_byte(0xFF24, 0x77); //NR50
        self.write_byte(0xFF25, 0xF3); //NR51
        self.write_byte(0xFF26, 0xF1); //GB(0xF1) or SGB(0xF0)
        self.write_byte(0xFF40, 0x91); //LCDC
        self.write_byte(0xFF42, 0x00); //SCY
        self.write_byte(0xFF43, 0x00); //SCX
        self.write_byte(0xFF45, 0x00); //LYC
        self.write_byte(0xFF47, 0xFC); //BGP
        self.write_byte(0xFF48, 0xFF); //OBP0
        self.write_byte(0xFF49, 0xFF); //OBP1
        self.write_byte(0xFF4A, 0x00); //WY
        self.write_byte(0xFF4B, 0x00); //WX
        self.write_byte(0xFFFF, 0x00); //IE
        self.write_byte(0xFF0F, 0xE1); //IF
    }
    pub fn draw(&mut self, canvas: &mut Canvas<Window>) -> bool {
        self.gpu.draw(canvas)
    }
    pub fn tick(&mut self, clock_t: u32) {
        self.update_special_registers();
        let interrupts = self.gpu.tick(clock_t);
        if interrupts > 0 {
            self.interupt_flag |= interrupts;
        }
    }

    fn add_to_div(&mut self, amount: u8) {
        self.div_register.val = self.div_register.val.wrapping_add(amount);
    }

    pub fn update_special_registers(&mut self) -> bool {
        // joypad
        self.add_to_div(1); // TODO: figure out correct timing

        // timers
        let mut timer_add = 0;
        if (self.timer_control & (1 << 2)) == (1 << 2) {
            let config = self.timer_control & 0b11;
            match config {
                0x00 => timer_add = 1,  //  4.096 KHz
                0x01 => timer_add = 64, //  262.144 Khz
                0x10 => timer_add = 16, //  65.536 KHz
                0x11 => timer_add = 4,  //  16.384 KHz
                _ => panic!(),
            }
        }
        let (new_val, overflow) = self.timer_counter.overflowing_add(timer_add);
        if overflow {
            self.timer_counter = self.timer_modulo;
            self.interupt_flag |= 1 << 2;
        } else {
            self.timer_counter = new_val;
        }

        return overflow;
    }

    fn color_to_char(color: &video::GBColor) -> String {
        match color {
            video::GBColor::White => "W".to_string(),
            video::GBColor::LightGray => "L".to_string(),
            video::GBColor::DarkGray => "D".to_string(),
            video::GBColor::Black => "B".to_string(),
        }
    }

    pub(crate) fn write_bg_tiles_to_file(&self) {
        let bg_tiles = self.gpu.get_bg_tiles();
        let mut file = File::create("bg_tiles.txt").unwrap();
        for i in (0..bg_tiles.len()).step_by(4) {
            file.write(
                format!(
                    "Tile {0} \t Tile {1} \t Tile {2} \t Tile {3}\n",
                    i,
                    i + 1,
                    i + 2,
                    i + 3
                )
                .as_bytes(),
            ).unwrap();
            for y in 0..8 {
                for j in i..i + 4 {
                    for x in 0..8 {
                        file.write(
                            format!("{}", Self::color_to_char(&bg_tiles[j][y][x])).as_bytes(),
                        ).unwrap();
                    }
                    file.write("\t".as_bytes()).unwrap();
                }
                file.write("\n".as_bytes()).unwrap();
            }
            file.write("\n".as_bytes()).unwrap();
        }

        file.write("tilemap 0x9800\n".as_bytes()).unwrap();
        for i in (0x9800..=0x9fff - 10).step_by(10) {
            for j in 0..10 {
                file.write(format!("{0},", self.gpu.read_byte(i + j)).as_bytes()).unwrap();
            }
            if i + 10 < 0x9fff {
                file.write(format!("\ntilemap {0:#0x}\n", i + 10).as_bytes()).unwrap();
            }
        }
        file.write("\n".as_bytes()).unwrap();

        for y in 0..SCREEN_HEIGHT {
            for x in 0..SCREEN_WIDTH {
                file.write(
                    format!("{}", Self::color_to_char(&self.gpu.get_pixel(x, y))).as_bytes(),
                ).unwrap();
            }
            file.write("\n".as_bytes()).unwrap();
        }

        println!("tiles dumped to bg_tiles.txt");
    }

    pub fn dump_bg_tiles(&self) ->  &[[[video::GBColor; 8]; 8]; 384] {
        self.gpu.get_bg_tiles()
    }
}
