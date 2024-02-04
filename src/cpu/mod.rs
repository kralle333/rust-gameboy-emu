mod cpu_test;
mod execute;
mod execute_cb;
mod helpers;
mod helpers_cb;

use std::ops::{Shl, Shr};

use crate::memory::{self, MemoryType};

#[derive(Default)]
pub enum Instruction {
    #[default]
    None,
    Ok(u8, u16, u32, &'static str),
    Invalid(u8),
}

#[allow(non_snake_case)]
#[derive(Default)]
pub struct Cpu {
    AF: u16,
    BC: u16,
    DE: u16,
    HL: u16,
    SP: u16,
    PC: u16,

    IME: bool,
    HALT: bool,

    DI: bool,
    EI: bool,
    clock_m: u8,
    clock_t: u32,

    triggered_interruption: String,
    last_instruction: Instruction,
    last_regs: String,
}

#[derive(Clone, Copy)]
pub enum Register {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    F,
}

#[derive(PartialEq)]
pub enum Flag {
    Z = 0x80,
    N = 0x40,
    H = 0x20,
    C = 0x10,
}

impl Cpu {
    pub fn new() -> Cpu {
        let mut c = Cpu::default();
        c.reset();
        c
    }

    pub fn reset(&mut self) {
        self.AF = 0x01B0;
        self.BC = 0x0013;
        self.DE = 0x00D8;
        self.HL = 0x014D;
        self.SP = 0xFFFE;
        self.PC = 0x0100;
        self.clock_m = 0;
        self.clock_t = 1;
        self.IME = false;
        self.HALT = false;
        self.DI = false;
        self.EI = false;
        self.last_instruction = Instruction::None;
        self.last_regs = "".to_string();
        self.triggered_interruption = "".to_string();
    }

    pub fn tick(&mut self, mem: &mut memory::Memory) {
        if self.PC == 0x0100 {
            mem.set_out_of_bios();
        }
        self.fetch_decode(mem);
    }

    pub fn print(&self) {
        if self.HALT {
            return;
        }
        match self.last_instruction {
            Instruction::None => {}
            Instruction::Ok(opcode, _, _, description) => {
                println!(
                    "{0:010}|op:{1} {2}",
                    description,
                    Self::clean_hex_8(opcode),
                    self.last_regs,
                );
            }
            Instruction::Invalid(opcode) => {
                println!("invalid opcode! {opcode:#06x}");
            }
        }
        if !self.triggered_interruption.is_empty() {
            println!("Interrupt triggered: {}", self.triggered_interruption);
        }
    }

    pub fn get_clock_t(&self) -> u32 {
        self.clock_t
    }

    #[allow(non_snake_case)]
    pub fn PC(&self) -> u16 {
        self.PC
    }

    fn get_a(&self) -> u8 {
        Self::get_upper(self.AF)
    }
    fn get_f(&self) -> u8 {
        Self::get_lower(self.AF)
    }
    fn get_b(&self) -> u8 {
        Self::get_upper(self.BC)
    }
    fn get_c(&self) -> u8 {
        Self::get_lower(self.BC)
    }
    fn get_d(&self) -> u8 {
        Self::get_upper(self.DE)
    }
    fn get_e(&self) -> u8 {
        Self::get_lower(self.DE)
    }
    fn get_h(&self) -> u8 {
        Self::get_upper(self.HL)
    }
    fn get_l(&self) -> u8 {
        Self::get_lower(self.HL)
    }

    fn set_reg(&mut self, register: Register, val: u8) {
        match register {
            Register::A => self.set_a(val),
            Register::F => self.set_f(val),
            Register::B => self.set_b(val),
            Register::C => self.set_c(val),
            Register::D => self.set_d(val),
            Register::E => self.set_e(val),
            Register::H => self.set_h(val),
            Register::L => self.set_l(val),
        }
    }
    fn get_reg(&self, register: Register) -> u8 {
        match register {
            Register::A => self.get_a(),
            Register::F => self.get_f(),
            Register::B => self.get_b(),
            Register::C => self.get_c(),
            Register::D => self.get_d(),
            Register::E => self.get_e(),
            Register::H => self.get_h(),
            Register::L => self.get_l(),
        }
    }
    fn get_upper(reg: u16) -> u8 {
        (reg & 0xff00).shr(8) as u8
    }
    fn get_lower(reg: u16) -> u8 {
        (reg & 0x00ff) as u8
    }
    fn set_upper(reg: u16, val: u8) -> u16 {
        (reg & 0x00ff) | (val as u16).shl(8)
    }
    fn set_lower(reg: u16, val: u8) -> u16 {
        (reg & 0xff00) | (val as u16)
    }
    fn set_a(&mut self, val: u8) {
        self.AF = Self::set_upper(self.AF, val);
    }
    fn set_f(&mut self, val: u8) {
        self.AF = Self::set_lower(self.AF, val);
    }
    fn set_b(&mut self, val: u8) {
        self.BC = Self::set_upper(self.BC, val);
    }
    fn set_c(&mut self, val: u8) {
        self.BC = Self::set_lower(self.BC, val);
    }
    fn set_d(&mut self, val: u8) {
        self.DE = Self::set_upper(self.DE, val);
    }
    fn set_e(&mut self, val: u8) {
        self.DE = Self::set_lower(self.DE, val);
    }
    fn set_h(&mut self, val: u8) {
        self.HL = Self::set_upper(self.HL, val);
    }
    fn set_l(&mut self, val: u8) {
        self.HL = Self::set_lower(self.HL, val);
    }

    fn set_flag(&mut self, flag: Flag, val: bool) {
        if val {
            self.AF = self.AF | (flag as u16);
        } else {
            self.AF = self.AF & !(flag as u16);
        }
    }
    fn reset_all_flags(&mut self) {
        self.AF = self.AF & 0xff00;
    }

    fn get_flag(&self, flag: Flag) -> bool {
        let flag = flag as u16;
        (self.AF & flag) == flag
    }

    fn set_clocks(&mut self, m: u8, t: u32) {
        self.clock_m = m;
        self.clock_t = t;
    }

    fn fetch_decode(&mut self, mem: &mut memory::Memory) {
        if !self.HALT {
            let opcode = mem.read_byte(self.PC);
            self.last_regs = self.registers_str(&mem);
            self.last_instruction = match opcode {
                0xcb => self.execute_cb(mem.read_byte(self.PC.wrapping_add(1)), mem),
                _ => self.execute(opcode, mem),
            };
            match self.last_instruction {
                Instruction::None => {}
                Instruction::Ok(_, length, clocks, _) => {
                    self.set_clocks(0, clocks);
                    self.PC = self.PC.wrapping_add(length);
                }
                Instruction::Invalid(opcode) => println!("invalid opcode {}", Self::clean_hex_8(opcode)),
            }
        }
        if let Instruction::Ok(last_op, _, _, _) = self.last_instruction {
            self.check_interrupt_status(mem, last_op);
        }
    }

    fn check_interrupt_status(&mut self, mem: &mut memory::Memory, last_opcode: u8) {
        self.triggered_interruption = "".to_string();

        //Go through the five different interrupts and see if any is triggered
        if self.DI && last_opcode & 0xf3 != last_opcode {
            self.DI = false;
            self.IME = false;
        }
        if self.EI && last_opcode & 0xfb != last_opcode {
            self.EI = false;
            self.IME = true;
        }
        if !self.IME {
            return;
        }

        let enabled_interrupts = mem.read_byte(0xFFFF);
        let interrupt_flag = mem.read_byte(0xFF0F);

        let to_fire = enabled_interrupts & interrupt_flag;

        for i in 0..=4 {
            let interrupt = to_fire & (1 << i);
            if interrupt == 0 {
                continue;
            }
            let restart_address: u16;
            match i {
                0 => restart_address = 0x40,
                1 => restart_address = 0x48,
                2 => restart_address = 0x50,
                3 => restart_address = 0x58,
                4 => restart_address = 0x60,
                _ => panic!("unknown flag"),
            }
            let interrupt_name = match i {
                0 => "LCD vertical blanking impulse",
                1 => "LY=LYC",
                2 => "Timer overflow",
                3 => "End of serial I/O transfer",
                4 => "Transition High->Low on pins P10-P13",
                _ => panic!("unknown flag")
            };
            self.triggered_interruption = interrupt_name.to_string();
            mem.write_byte(0xFF0F, interrupt_flag & !(1 << i));
            self.rst(mem, restart_address);
            return;
        }
    }

    fn clean_hex_8(v: u8) -> String {
        format!("{0:#04x}", v).replace("0x", "")
    }
    fn clean_hex_16(v: u16) -> String {
        format!("{0:#06x}", v).replace("0x", "")
    }
    fn clean_b_8(v: u8) -> String {
        format!("{0:#06b}", v >> 4).replace("0b", "")
    }
    fn registers_str(&self, mem: &memory::Memory) -> String {
        let mut s;
        s = format!("PC:{0}", Self::clean_hex_16(self.PC));
        s = format!("{s} SP:{0}", Self::clean_hex_16(self.SP));
        s = format!("{s} A:{0}", Self::clean_hex_8(self.get_a()));
        s = format!("{s} F:{0}", Self::clean_b_8(self.get_f()));
        s = format!("{s} B:{0}", Self::clean_hex_8(self.get_b()));
        s = format!("{s} C:{0}", Self::clean_hex_8(self.get_c()));
        s = format!("{s} D:{0}", Self::clean_hex_8(self.get_d()));
        s = format!("{s} E:{0}", Self::clean_hex_8(self.get_e()));
        s = format!("{s} H:{0}", Self::clean_hex_8(self.get_h()));
        s = format!("{s} L:{0}", Self::clean_hex_8(self.get_l()));
        s = format!("{s} nn:{0}", Self::clean_hex_16(self.get_nn(mem)));
        s
    }
}
