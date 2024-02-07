mod gpu;
mod gpu_test;
mod mem_test;
mod rom;
mod sound;

use std::{fs::File, io::Write, ops::Shl};

use sdl2::render::{Canvas, Texture};
use sdl2::video::Window;

use crate::{
    cartridge::Cartridge,
    video::{SCREEN_HEIGHT, SCREEN_WIDTH, self},
};

use self::{gpu::{Gpu}, rom::Rom, sound::Sound};

#[allow(dead_code)]
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

#[allow(dead_code)]
struct DivRegister {
    val: u8,
    added_this_second: u8,
    time_since_last_check: u16,
}

pub struct Memory {
    rom: Rom,
    gpu: Gpu,
    snd: Sound,
    interupt_enable: u8,
    interupt_flag: u8,
    bios: [u8; 0x0100],
    in_bios: bool,

    //special registers
    // FF00
    joypad: u8,
    //FF01
    serial_transfer_data: u8,
    //FF02
    serial_transfer_control: u8,
    //FF04
    div_register: DivRegister,
    //FF05
    timer_counter: u8,
    //FF06
    timer_modulo: u8,
    //FF07
    timer_control: u8,
}

impl MemoryType for Memory {
    fn read_byte(&self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x00ff => {
                if self.in_bios {
                    return self.bios[addr as usize];
                }
                self.rom.read_byte(addr)
            }
            0x0100..=0x7fff => self.rom.read_byte(addr),
            0x8000..=0x9fff => self.gpu.read_byte(addr),
            0xa000..=0xfdff => self.rom.read_byte(addr),
            0xfe00..=0xfe9f => self.gpu.read_byte(addr),
            0xfea0..=0xfeff => {
                //println!("Reading from empty but unusable for I/O: 0xfea0-0xff00 {}",addr);
                0
            }
            0xff00 => self.joypad,
            0xff01 => self.serial_transfer_data,
            0xff02 => self.serial_transfer_control,
            0xff04 => self.div_register.val,
            0xff05 => self.timer_counter,
            0xff06 => self.timer_modulo,
            0xff07 => self.timer_control,
            0xff0f => self.interupt_flag,
            0xff10..=0xff3f => self.snd.read_byte(addr),
            0xff40..=0xff4b => self.gpu.read_byte(addr),
            0xff80..=0xfffe => self.rom.read_byte(addr),
            0xff4c..=0xff7f => {
                //println!("Reading from empty but unusable for I/O: 0xff4c-0xff80 {}",addr);
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
            0x8000..=0x9fff => {
                if addr == 0xff40 { // DMA transfer
                    let start = (val as u16) << 8;
                    for i in 0..140u16 {
                        let from_addr = start + i;
                        let to_addr = 0xfe00 + i;
                        self.write_byte(to_addr, self.read_byte(from_addr));
                    }
                    return;
                }
                self.gpu.write_byte(addr, val);
            }
            0xa000..=0xfdff => self.rom.write_byte(addr, val),
            0xfe00..=0xfe9f => self.gpu.write_byte(addr, val),
            0xfea0..=0xfeff => {}
            0xff00 => self.joypad = val,
            0xff01 => self.serial_transfer_data = val,
            0xff02 => {
                self.serial_transfer_control = val;
                // BLARGG
                if val == 0x81 {
                    print!("{}", self.serial_transfer_data as char);
                    self.serial_transfer_control = 0;
                }
            }
            0xff04 => self.div_register.val = 0,
            0xff05 => self.timer_counter = val,
            0xff06 => self.timer_modulo = val,
            0xff07 => self.timer_control = val,
            0xff0f => self.interupt_flag = val,
            0xff10..=0xff3f => self.snd.write_byte(addr, val),
            0xff40..=0xff4b => self.gpu.write_byte(addr, val),
            0xff4c..=0xff7f => {}
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
            gpu: Gpu::new(),
            snd: Sound::new(),
            bios: [
                0x31, 0xFE, 0xFF, 0xAF, 0x21, 0xFF, 0x9F, 0x32, 0xCB, 0x7C, 0x20, 0xFB, 0x21, 0x26, 0xFF, 0x0E,
                0x11, 0x3E, 0x80, 0x32, 0xE2, 0x0C, 0x3E, 0xF3, 0xE2, 0x32, 0x3E, 0x77, 0x77, 0x3E, 0xFC, 0xE0,
                0x47, 0x11, 0x04, 0x01, 0x21, 0x10, 0x80, 0x1A, 0xCD, 0x95, 0x00, 0xCD, 0x96, 0x00, 0x13, 0x7B,
                0xFE, 0x34, 0x20, 0xF3, 0x11, 0xD8, 0x00, 0x06, 0x08, 0x1A, 0x13, 0x22, 0x23, 0x05, 0x20, 0xF9,
                0x3E, 0x19, 0xEA, 0x10, 0x99, 0x21, 0x2F, 0x99, 0x0E, 0x0C, 0x3D, 0x28, 0x08, 0x32, 0x0D, 0x20,
                0xF9, 0x2E, 0x0F, 0x18, 0xF3, 0x67, 0x3E, 0x64, 0x57, 0xE0, 0x42, 0x3E, 0x91, 0xE0, 0x40, 0x04,
                0x1E, 0x02, 0x0E, 0x0C, 0xF0, 0x44, 0xFE, 0x90, 0x20, 0xFA, 0x0D, 0x20, 0xF7, 0x1D, 0x20, 0xF2,
                0x0E, 0x13, 0x24, 0x7C, 0x1E, 0x83, 0xFE, 0x62, 0x28, 0x06, 0x1E, 0xC1, 0xFE, 0x64, 0x20, 0x06,
                0x7B, 0xE2, 0x0C, 0x3E, 0x87, 0xF2, 0xF0, 0x42, 0x90, 0xE0, 0x42, 0x15, 0x20, 0xD2, 0x05, 0x20,
                0x4F, 0x16, 0x20, 0x18, 0xCB, 0x4F, 0x06, 0x04, 0xC5, 0xCB, 0x11, 0x17, 0xC1, 0xCB, 0x11, 0x17,
                0x05, 0x20, 0xF5, 0x22, 0x23, 0x22, 0x23, 0xC9, 0xCE, 0xED, 0x66, 0x66, 0xCC, 0x0D, 0x00, 0x0B,
                0x03, 0x73, 0x00, 0x83, 0x00, 0x0C, 0x00, 0x0D, 0x00, 0x08, 0x11, 0x1F, 0x88, 0x89, 0x00, 0x0E,
                0xDC, 0xCC, 0x6E, 0xE6, 0xDD, 0xDD, 0xD9, 0x99, 0xBB, 0xBB, 0x67, 0x63, 0x6E, 0x0E, 0xEC, 0xCC,
                0xDD, 0xDC, 0x99, 0x9F, 0xBB, 0xB9, 0x33, 0x3E, 0x3c, 0x42, 0xB9, 0xA5, 0xB9, 0xA5, 0x42, 0x4C,
                0x21, 0x04, 0x01, 0x11, 0xA8, 0x00, 0x1A, 0x13, 0xBE, 0x20, 0xFE, 0x23, 0x7D, 0xFE, 0x34, 0x20,
                0xF5, 0x06, 0x19, 0x78, 0x86, 0x23, 0x05, 0x20, 0xFB, 0x86, 0x20, 0xFE, 0x3E, 0x01, 0xE0, 0x50
            ],
            in_bios: true,
            interupt_enable: 0,
            interupt_flag: 0,
            joypad: 0,
            serial_transfer_data: 0,
            serial_transfer_control: 0,
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
    pub fn draw_texture(&mut self, texture: &mut Texture) -> bool {
        self.gpu.draw_texture(texture)
    }
    pub fn tick(&mut self, clock_t: u32) {
        self.update_special_registers(clock_t);
        let interrupts = self.gpu.tick(clock_t);
        if interrupts > 0 {
            //println!("Setting interrupts: {interrupts}");
            self.interupt_flag |= interrupts;
        }
    }

    fn add_to_div(&mut self, amount: u8) {
        self.div_register.val = self.div_register.val.wrapping_add(amount);
    }
    fn is_bit_set(val: u8, bit: u8) -> bool {
        return (val & (1 << bit)) == (1 << bit);
    }
    pub fn update_special_registers(&mut self, clock_t: u32) -> bool {
        // joypad
        self.add_to_div(1); // TODO: figure out correct timing

        let clock_m = clock_t/4;
        // timers
        let mut timer_add = 0;
        if Self::is_bit_set(self.timer_control, 2) {
            match self.timer_control & 0b11 {
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

    pub(crate) fn in_bios(&self) -> bool {
        self.in_bios
    }

    pub(crate) fn set_out_of_bios(&mut self) {
        self.in_bios = false;
    }

    pub(crate) fn write_bg_tiles_to_file(&self) {
        let bg_tiles = self.gpu.get_tiles();
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

    pub fn dump_tiles(&self) -> &[[[video::GBColor; 8]; 8]; 384] {
        self.write_bg_tiles_to_file();
        self.gpu.get_tiles()
    }
    pub fn debug_get_background_tilemap(&self) -> [u8; 32 * 32] {
        self.gpu.debug_get_background_tilemap()
    }
    pub(crate) fn debug_toggle_background(&mut self) { self.gpu.debug_toggle_background() }
    pub(crate) fn debug_toggle_window(&mut self) { self.gpu.debug_toggle_window() }
    pub(crate) fn debug_toggle_objects(&mut self) { self.gpu.debug_toggle_objects() }
}
