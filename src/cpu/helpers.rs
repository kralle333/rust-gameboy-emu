use crate::memory::{self, Memory, MemoryType};

use super::{Cpu, Flag, Register};

impl Cpu {
    pub fn get_n(&self, mem: &mut memory::Memory) -> u8 {
        mem.read_byte(self.PC + 1)
    }
    pub fn get_nn(&self, mem: &memory::Memory) -> u16 {
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

    pub fn add_a(&mut self, b: u8) {
        let a = self.get_a();
        let (new_a, carry) = Self::overflow_add(a, b);
        self.set_a(new_a);
        self.set_flag(Flag::Z, new_a == 0);
        self.set_flag(Flag::N, false);
        self.set_flag(Flag::H, Self::is_half_carry_add(a, b));
        self.set_flag(Flag::C, carry);
    }
    pub fn adc_a(&mut self, b: u8) {
        self.add_a(b + self.get_flag(Flag::C) as u8);
    }
    pub fn sub_a(&mut self, b: u8) {
        let a = self.get_a();
        let (new_a, borrow) = Self::borrow_sub(a, b);

        self.set_a(new_a);
        self.set_flag(Flag::Z, new_a == 0);
        self.set_flag(Flag::N, true);
        self.set_flag(Flag::H, Self::is_half_borrowing_sub(a, b));
        self.set_flag(Flag::C, borrow);
    }
    pub fn sbc_a(&mut self, b: u8) {
        self.sub_a(b + self.get_flag(Flag::C) as u8);
    }

    pub fn add_16(&mut self, reg: u16, val: u16) -> u16 {
        let (result, overflow) = Self::overflow_add_16(reg, val);
        self.set_flag(Flag::H, Self::is_half_carry_on_add_16(reg, val));
        self.set_flag(Flag::C, overflow);
        self.set_flag(Flag::N, false);
        result
    }
    pub fn sub_16(&mut self, reg: u16, val: u16) -> u16 {
        let (result, overflow) = Self::borrow_sub_16(reg, val);
        self.set_flag(Flag::H, Self::is_half_carry_on_sub_16(reg, val));
        self.set_flag(Flag::C, overflow);
        self.set_flag(Flag::N, false);
        result
    }

    pub fn and_a(&mut self, b: u8) {
        let a = self.get_a();
        let result = a & b;
        self.set_a(result);
        self.set_flag(Flag::Z, result == 0);
        self.set_flag(Flag::N, false);
        self.set_flag(Flag::H, true);
        self.set_flag(Flag::C, false);
    }
    pub fn xor_a(&mut self, b: u8) {
        let a = self.get_a();
        let result = a ^ b;
        self.set_a(result);
        self.set_flag(Flag::Z, result == 0);
        self.set_flag(Flag::N, false);
        self.set_flag(Flag::H, false);
        self.set_flag(Flag::C, false);
    }
    pub fn or_a(&mut self, b: u8) {
        let a = self.get_a();
        let result = a | b;
        self.set_a(result);
        self.set_flag(Flag::Z, result == 0);
        self.set_flag(Flag::N, false);
        self.set_flag(Flag::H, false);
        self.set_flag(Flag::C, false);
    }
    pub fn cp_a(&mut self, b: u8) {
        let a = self.get_a();
        self.sub_a(b);
        self.set_a(a);
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
        let mut val = self.get_reg(register);
        let half_borrow = Self::is_half_borrowing_sub(val, 1);
        val = val.wrapping_sub(1);
        self.set_reg(register, val);

        self.set_flag(Flag::H, half_borrow);
        self.set_flag(Flag::N, true);
        self.set_flag(Flag::Z, val == 0);
    }

    pub fn pop_sp(&mut self, mem: &Memory) -> u16 {
        let out = mem.read_word(self.SP);
        self.SP += 2;
        out
    }
    pub fn push_sp(&mut self, mem: &mut Memory, rcv: u16) {
        self.SP -= 2;
        mem.write_word(self.SP, rcv);
    }

    pub fn call(&mut self, mem: &mut Memory) {
        self.push_sp(mem, self.PC);
        self.PC = self.get_nn(mem);
    }

    pub fn rst(&mut self, mem: &mut Memory, addr: u16) {
        self.push_sp(mem, self.PC);
        self.PC = addr;
        self.IME = false;
        self.HALT = false;
    }

    pub fn inc_register(&mut self, register: Register) {
        let mut val = self.get_reg(register);
        let borrow = Self::is_half_carry_add(val, 1);
        val = val.wrapping_add(1);
        self.set_reg(register, val);

        self.set_flag(Flag::H, borrow);
        self.set_flag(Flag::N, false);
        self.set_flag(Flag::Z, val == 0)
    }

    pub fn is_half_carry_add(a: u8, b: u8) -> bool {
        ((a & 0x0F) + (b & 0x0F)) > 0xF
    }
    pub fn is_half_borrowing_sub(b: u8, a: u8) -> bool {
        (b & 0xF) < (a & 0xF)
    }

    pub fn is_half_carry_on_add_16(a: u16, b: u16) -> bool {
        (a & 0xFFF) + (b & 0xFFF) > 0xFFF
    }
    pub fn is_half_carry_on_sub_16(b: u16, a: u16) -> bool {
        (a & 0xFFF) > (b & 0xFFF)
    }
}
