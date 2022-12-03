use crate::mem;

use super::{Cpu, Register};

pub enum Instruction {
    Info(u16, u8, &'static str),
    Invalid(u8),
    Unimplemented(u8),
}

impl Cpu {
    pub fn get_n(&self, mem: &mut mem::Memory) -> u8 {
        mem.read_byte(self.PC + 1)
    }
    pub fn get_nn(&self, mem: &mut mem::Memory) -> u16 {
        mem.read_word(self.PC + 1)
    }

    pub fn overflow_add(a: u8, b: u8) -> (u8, bool) {
        match a.checked_add(b) {
            Some(x) => (x, false),
            None => (a.wrapping_add(b), true),
        }
    }

    pub fn overflow_add_16(a: u16, b: u16) -> (u16, bool) {
        match a.checked_add(b) {
            Some(x) => (x, false),
            None => (a.wrapping_add(b), true),
        }
    }
    pub fn add_16(&mut self, reg: u16, val: u16) -> u16 {
        let (result, overflow) = Self::overflow_add_16(reg, val);
        self.set_fh(Self::is_half_carry_on_add_16(reg, val));
        self.set_fc(overflow);
        self.set_fn(false);
        result
    }
    pub fn sub_16(&mut self, reg: u16, val: u16) -> u16 {
        let (result, overflow) = Self::borrow_sub_16(reg, val);
        self.set_fh(Self::is_half_carry_on_sub_16(reg, val)); // TODO FIX
        self.set_fc(overflow);
        self.set_fn(false);
        result
    }
    pub fn borrow_sub(b: u8, a: u8) -> (u8, bool) {
        match b.checked_sub(a) {
            Some(x) => (x, false),
            None => (b.wrapping_sub(a), true),
        }
    }
    pub fn borrow_sub_16(a: u16, b: u16) -> (u16, bool) {
        match a.checked_sub(b) {
            Some(x) => (x, false),
            None => (a.wrapping_sub(b), true),
        }
    }

    pub fn dec_register(&mut self, register: Register) {
        // Decrement the value in the register
        let value = self.get(register) - 1;
        let borrow = value > 0;
    
        // Update the flags
        self.set_fh(!borrow);
        self.set_fn(true);
        self.set_fz(value == 0);
    
        // Set the new value in the register
        self.set(register, value)
    }

    pub fn inc_register(&mut self, register: Register) {
        let (value, carry) = Self::overflow_add(self.get(register), 1);
        if carry {
            self.set_fh(true);
        }
        self.set_fn(false);
        if self.get_b() == 0 {
            self.set_fz(false)
        }
        self.set(register, value)
    }

    pub fn is_half_carry_on_add_16(a: u16, b: u16) -> bool {
        (a & 0xFFF) + (b & 0xFFF) > 0xFFF
    }
    pub fn is_half_carry_on_sub_16(a: u16, b: u16) -> bool { //TODO: DOUBLE CHECK
        (a & 0xFFF).wrapping_sub(b & 0xFFF) < a
    }
    //Bit setting/getting
    fn set_bits(var: u8, value: u8, pos: u8, len: u8) -> u8 {
        (var & !(len << pos)) | (value & len) << pos
    }
    fn get_bits(var: u8, pos: u8, len: u8) -> u8 {
        (var >> ((pos + 1) - len)) & len
    }
    fn set_bit(var: u8, value: u8, pos: u8) -> u8 {
        Self::set_bits(var, value, pos, 1)
    }
    fn get_bit(var: u8, pos: u8) -> u8 {
        var >> pos & 1
    }

    //Rotations
    fn rotate_left(a: u8, n: u8) -> u8 {
        (a << n) | (a >> (8 - n))
    }
    fn rotate_right(a: u8, n: u8) -> u8 {
        (a >> n) | (a << (8 - n))
    }
    fn swap_hexits(a: u8) -> u8 {
        Self::rotate_right(a, 4)
    }
}
