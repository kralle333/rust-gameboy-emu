mod gpu;
mod gpu_test;
mod mbc;
mod mem_test;
mod mmu;

use std::ops::Shl;

use sdl2::{render::Canvas, video::Window};


use self::{gpu::Gpu, mbc::Mbc, mmu::Mmu};

const divider_add: i16 = 16384;

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
    mbc: Mbc,
    mmu: Mmu,
    gpu: Gpu,
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
            0x0000..=0x7fff => self.mbc.read_byte(addr),
            0x8000..=0x9fff => self.gpu.read_byte(addr),
            0xa000..=0xfdff => self.mmu.read_byte(addr),
            0xfe00..=0xfe9f => self.gpu.read_byte(addr),
            0xfea0..=0xfeff => panic!("invalid address"),
            0xff00 => self.joypad,
            0xff01 => self.serial_transfer_data,
            0xff02 => self.serial_transer_control,
            0xff04 => self.div_register.val,
            0xff05 => self.timer_counter,
            0xff06 => self.timer_modulo,
            0xff07 => self.timer_control,
            0xff0f => self.interupt_flag,
            0xff10..=0xff3f => 0, // TODO: SOUND
            0xff40..=0xff4b => self.gpu.read_byte(addr),
            0xff80..=0xfffe => self.mmu.read_byte(addr),
            0xff4c..=0xfffe => panic!("invalid"),
            0xffff => self.interupt_enable,
            _ => panic!(),
        }
    }

    fn write_byte(&mut self, addr: u16, val: u8) {
        match addr {
            0x0000..=0x7fff => self.mbc.write_byte(addr, val),
            0x8000..=0x9fff => self.gpu.write_byte(addr, val),
            0xa000..=0xfdff => self.mmu.write_byte(addr, val),
            0xfe00..=0xfe9f => self.gpu.write_byte(addr, val),
            0xfea0..=0xfeff => panic!("invalid address"),
            0xff00 => self.joypad = val,
            0xff01 => self.serial_transfer_data = val,
            0xff02 => self.serial_transer_control = val,
            0xff04 => self.div_register.val = 0,
            0xff05 => self.timer_counter = val,
            0xff06 => self.timer_modulo = val,
            0xff07 => self.timer_control = val,
            0xff0f => self.interupt_flag = val,
            0xff10..=0xff3f => {} // TODO: SOUND
            0xff40..=0xff4b => self.gpu.write_byte(addr, val),
            0xff80..=0xfffe => self.mmu.write_byte(addr, val),
            0xff4c..=0xfffe => panic!("invalid"),
            0xffff => self.interupt_enable = val,
            _ => panic!("{}", addr),
        }
    }
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            mbc: Mbc::new(),
            mmu: Mmu::new(),
            gpu: Gpu::new(),
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
        }
    }

    pub fn load(&mut self, data: Vec<u8>) {
        self.mbc.load(&data);
    }
    pub fn reset(&mut self) {
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
        self.write_byte(0xFF26, 0xF1);
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
    pub fn draw(&mut self, canvas: &mut Canvas<Window>) {
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
}
