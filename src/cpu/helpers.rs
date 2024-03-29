use crate::memory::{Memory, MemoryType};

use super::{Cpu, Flag, Register};

impl Cpu {
    pub fn get_n(&self, mem: &mut Memory) -> u8 {
        mem.read_byte(self.PC + 1)
    }
    pub fn get_nn(&self, mem: &Memory) -> u16 {
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
        let before_a = self.get_a();
        let carry_val = self.get_flag(Flag::C) as u8;

        let half_carry = ((before_a & 0x0F) + (b & 0x0F) + (carry_val & 0x0F)) > 0xF;
        let carry = ((before_a as u16) + (b as u16) + (carry_val as u16)) > 0xFF;

        let result = before_a.wrapping_add(b).wrapping_add(carry_val);

        self.set_flag(Flag::Z, result == 0);
        self.set_flag(Flag::N, false);
        self.set_flag(Flag::H, half_carry);
        self.set_flag(Flag::C, carry);

        self.set_a(result);
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
        let before_a = self.get_a();
        let carry = self.get_flag(Flag::C) as u8;

        let result = before_a.wrapping_sub(b).wrapping_sub(carry);

        let borrow = (before_a as u16) < (b as u16 + carry as u16);
        let half_borrow =
            before_a & 0xF < b & 0xF ||
                before_a & 0xF < carry ||
                before_a & 0xF == b & 0xF && carry == 1;

        self.set_a(result);
        self.set_flag(Flag::N, true);
        self.set_flag(Flag::Z, result == 0);
        self.set_flag(Flag::H, half_borrow);
        self.set_flag(Flag::C, borrow);
    }

    pub fn add_sp_16_signed(&mut self, val: u8) {
        let sp_as_signed = self.SP as i32;
        let n_as_signed = val as i8;
        let n_as_signed_i32 = n_as_signed as i32;
        let result = sp_as_signed.wrapping_add(n_as_signed_i32);

        self.set_flag(Flag::Z, false);
        self.set_flag(Flag::N, false);
        self.set_flag(Flag::H, (sp_as_signed ^ n_as_signed_i32 ^ result) & 0x10 == 0x10);
        self.set_flag(Flag::C, (sp_as_signed ^ n_as_signed_i32 ^ result) & 0x100 == 0x100);
        self.SP = result as u16;
    }

    pub fn add_16(&mut self, reg: u16, val: u16) -> u16 {
        let (result, overflow) = Self::overflow_add_16(reg, val);
        self.set_flag(Flag::N, false);
        self.set_flag(Flag::H, Self::is_half_carry_on_add_16(reg, val));
        self.set_flag(Flag::C, overflow);
        result
    }
    pub fn sub_16(&mut self, reg: u16, val: u16) -> u16 {
        let (result, overflow) = Self::borrow_sub_16(reg, val);
        self.set_flag(Flag::N, false);
        self.set_flag(Flag::H, Self::is_half_carry_on_sub_16(reg, val));
        self.set_flag(Flag::C, overflow);
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
    pub fn jump_relative(&mut self, amount: i8) {
        let result = (self.PC + 2).wrapping_add(amount as u16);
        self.PC = result as u16;
    }
    pub fn jump_relative_with_flag(&mut self, amount: i8, flag: Flag, flag_set: bool) -> bool {
        if flag_set != self.get_flag(flag) {
            return false;
        }
        self.jump_relative(amount);
        return true;
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
    pub fn dec_8(&mut self, val: u8) -> u8 {
        let mut val = val;
        let half_borrow = Self::is_half_borrowing_sub(val, 1);
        val = val.wrapping_sub(1);

        self.set_flag(Flag::Z, val == 0);
        self.set_flag(Flag::N, true);
        self.set_flag(Flag::H, half_borrow);
        val
    }

    pub fn dec_register(&mut self, register: Register) {
        let val = self.get_reg(register);
        let result = self.dec_8(val);
        self.set_reg(register, result);
    }

    pub fn pop_sp(&mut self, mem: &Memory) -> u16 {
        let out = mem.read_word(self.SP);
        self.SP = self.SP.wrapping_add(2);
        out
    }
    pub fn push_sp(&mut self, mem: &mut Memory, rcv: u16) {
        self.SP = self.SP.wrapping_sub(2);
        mem.write_word(self.SP, rcv);
    }

    pub fn ret(&mut self, mem: &mut Memory) {
        self.PC = self.pop_sp(mem);
        if self.in_interrupt {
            self.in_interrupt = false;
        }
    }

    pub fn call_a16(&mut self, mem: &mut Memory) {
        self.push_sp(mem, self.PC + 3);
        self.PC = self.get_nn(mem);
    }

    pub fn rst(&mut self, mem: &mut Memory, addr: u16) {
        self.push_sp(mem, self.PC + 1);
        self.PC = addr;
        self.IME = false;
        self.HALT = false;
    }
    pub fn rst_interrupt(&mut self, mem: &mut Memory, addr: u16) {
        self.push_sp(mem, self.PC);
        self.PC = addr;
        self.IME = false;
        self.HALT = false;
    }

    pub fn inc_8(&mut self, val: u8) -> u8 {
        let mut val = val;
        let borrow = Self::is_half_carry_add(val, 1);
        val = val.wrapping_add(1);

        self.set_flag(Flag::Z, val == 0);
        self.set_flag(Flag::N, false);
        self.set_flag(Flag::H, borrow);
        val
    }

    pub fn inc_register(&mut self, register: Register) {
        let val = self.get_reg(register);
        let result = self.inc_8(val);
        self.set_reg(register, result);
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
