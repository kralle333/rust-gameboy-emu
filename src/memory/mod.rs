mod gpu;
mod mbc;
mod mmu;

use std::ops::{Shl, Shr};

use self::{gpu::Gpu, mbc::Mbc, mmu::Mmu};

pub trait MemoryType {
    fn read_byte(&self, addr: u16) -> u8;
    fn write_byte(&mut self, addr: u16, val: u8);
    fn read_word(&self, addr: u16) -> u16 {
        let lsn = self.read_byte(addr) as u16;
        let msn = (self.read_byte(addr + 1) as u16).shl(8);
        msn | lsn
    }
    fn write_word(&mut self, addr: u16, val: u16) {
        self.write_byte(addr, (val & 0xFF) as u8);
        self.write_byte(addr + 1, (val & 0xFF00).shr(8) as u8);
    }
}

pub struct Memory {
    mbc: Mbc,
    mmu: Mmu,
    gpu: Gpu,
    interupt_enable: u8,
    interupt_flag: u8,
}

impl MemoryType for Memory {
    fn read_byte(&self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x7fff => self.mbc.read_byte(addr),
            0x8000..=0x9fff => self.gpu.read_byte(addr),
            0xa000..=0xfdff => self.mmu.read_byte(addr),
            0xfe00..=0xfe9f => self.gpu.read_byte(addr),
            0xfea0..=0xfeff => panic!("invalid address"),
            0xff00..=0xff0e => 0, //FLAGS,
            0xff0f => self.interupt_flag,
            0xff10..=0xff3f => todo!("sound"),
            0xff40..=0xff4b => self.gpu.read_byte(addr),
            0xff80..=0xfffe => self.mmu.read_byte(addr),
            0xff4c..=0xfffe => panic!("invalid"),
            0xffff => self.interupt_enable,
        }
    }

    fn write_byte(&mut self, addr: u16, val: u8) {
        match addr {
            0x0000..=0x7fff => self.mbc.write_byte(addr, val),
            0x8000..=0x9fff => self.gpu.write_byte(addr, val),
            0xa000..=0xfdff => self.mmu.write_byte(addr, val),
            0xfe00..=0xfe9f => self.gpu.write_byte(addr, val),
            0xfea0..=0xfeff => panic!("invalid address"),
            0xff00..=0xff0e => todo!("flags"),
            0xff0f => self.interupt_flag = val,
            0xff10..=0xff3f => todo!("sound"),
            0xff40..=0xff4b => self.gpu.write_byte(addr, val),
            0xff80..=0xfffe => self.mmu.write_byte(addr, val),
            0xff4c..=0xfffe => panic!("invalid"),
            0xffff => self.interupt_enable = val,
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
        }
    }

    pub fn load(&mut self, data: Vec<u8>) {
        self.mbc.load(&data);
    }
}
