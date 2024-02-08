use std::ops::{Shl, Shr};

use crate::cpu::Cpu;

use super::Flag;

impl Cpu {
    pub fn rlc(&mut self, val: u8) -> u8 {
        self.reset_all_flags();
        self.set_flag(Flag::C, (0x80 & val) != 0);
        let result = val.rotate_left(1);
        self.set_flag(Flag::Z, result == 0);
        result
    }
    pub fn rrc(&mut self, val: u8) -> u8 {
        self.reset_all_flags();
        self.set_flag(Flag::C, (1 & val) != 0);
        let result = val.rotate_right(1);
        self.set_flag(Flag::Z, result == 0);
        result
    }

    pub fn rl(&mut self, val: u8) -> u8 {
        let result = val.shl(1) | (self.get_flag(Flag::C) as u8);
        self.reset_all_flags();
        self.set_flag(Flag::Z, result == 0);
        self.set_flag(Flag::C, (0x80 & val) != 0);
        result
    }
    pub fn rr(&mut self, val: u8) -> u8 {
        let result = val.shr(1) | (self.get_flag(Flag::C) as u8).shl(7);
        self.reset_all_flags();
        self.set_flag(Flag::Z, result == 0);
        self.set_flag(Flag::C, (1 & val) != 0);
        result
    }

    pub fn sla(&mut self, val: u8) -> u8 {
        let result = val.shl(1);
        self.reset_all_flags();
        self.set_flag(Flag::Z, result == 0);
        self.set_flag(Flag::C, (0x80 & val) != 0);
        result
    }
    pub fn sra(&mut self, val: u8) -> u8 {
        let result = val.shr(1) | (1 << 7 & val);
        self.reset_all_flags();
        self.set_flag(Flag::Z, result == 0);
        self.set_flag(Flag::C, (1 & val) != 0);
        result
    }

    pub fn swap(&mut self, val: u8) -> u8 {
        let result = (val >> 4) | (val << 4);
        self.reset_all_flags();
        self.set_flag(Flag::Z, result == 0);
        result
    }
    pub fn srl(&mut self, val: u8) -> u8 {
        let result = val.shr(1);
        self.reset_all_flags();
        self.set_flag(Flag::Z, result == 0);
        self.set_flag(Flag::C, (1 & val) != 0);
        result
    }
    pub fn bit(&mut self, b: u8, val: u8) {
        self.set_flag(Flag::Z, ((1 << b) & val) == 0);
        self.set_flag(Flag::N,false);
        self.set_flag(Flag::H,true);
    }
    pub fn res_bit(&mut self, b: u8, val: u8) -> u8 {
        (!(1 << b)) & val
    }
    pub fn set_bit(&mut self, b: u8, val: u8) -> u8 {
        ((1 << b)) | val
    }
}
