mod execute;
mod execute_cb;
mod helpers;

use crate::mem;

use self::helpers::Instruction;

#[allow(non_snake_case)]
pub struct Cpu {
    AF: u16,
    BC: u16,
    DE: u16,
    HL: u16,
    SP: u16,
    PC: u16,

    clock_m: u8,
    clock_t: u8,
}

#[derive(Clone, Copy)]
pub enum Register {
    A,
    B,
    C,
    D,
    E,
    F,
    H,
    L,
}

enum Opcode {
    Normal,
    CB,
}

impl std::fmt::Display for Opcode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Opcode::Normal => write!(f, "Normal"),
            Opcode::CB => write!(f, "CB"),
        }
    }
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            AF: 0x01B0,
            BC: 0x0013,
            DE: 0x00D8,
            HL: 0x014D,
            SP: 0xFFFE,
            PC: 0x100,
            clock_m: 0,
            clock_t: 1,
        }
    }

    pub fn tick(&mut self, mem: &mut mem::Memory) {
        self.fetch_decode(mem)
    }

    fn get_a(&self) -> u8 {
        (self.AF & 0x00ff) as u8
    }
    fn get_f(&self) -> u8 {
        (self.AF & 0xff00) as u8
    }
    fn get_b(&self) -> u8 {
        (self.BC & 0xff00) as u8
    }
    fn get_c(&self) -> u8 {
        (self.BC & 0x00ff) as u8
    }
    fn get_d(&self) -> u8 {
        (self.DE & 0xff00) as u8
    }
    fn get_e(&self) -> u8 {
        (self.DE & 0x00ff) as u8
    }
    fn get_h(&self) -> u8 {
        (self.HL & 0xff00) as u8
    }
    fn get_l(&self) -> u8 {
        (self.HL & 0x00ff) as u8
    }

    fn set(&mut self, register: Register, value: u8) {
        match register {
            Register::A => self.set_a(value),
            Register::B => self.set_b(value),
            Register::C => self.set_c(value),
            Register::D => self.set_d(value),
            Register::E => self.set_e(value),
            Register::F => self.set_f(value),
            Register::H => self.set_h(value),
            Register::L => self.set_l(value),
        }
    }
    fn get(&self, register: Register) -> u8 {
        match register {
            Register::A => self.get_a(),
            Register::B => self.get_b(),
            Register::C => self.get_c(),
            Register::D => self.get_d(),
            Register::E => self.get_e(),
            Register::F => self.get_f(),
            Register::H => self.get_h(),
            Register::L => self.get_l(),
        }
    }
    

    fn set_a(&mut self, value: u8) {
        self.AF = (self.AF & 0x00ff) & value as u16
    }
    fn set_f(&mut self, value: u8) {
        self.AF = (self.AF & 0xff00) & value as u16
    }
    fn set_b(&mut self, value: u8) {
        self.BC = (self.BC & 0x00ff) & value as u16
    }
    fn set_c(&mut self, value: u8) {
        self.BC = (self.BC & 0xff00) & value as u16
    }
    fn set_d(&mut self, value: u8) {
        self.DE = (self.DE & 0x00ff) & value as u16
    }
    fn set_e(&mut self, value: u8) {
        self.DE = (self.DE & 0xff00) & value as u16
    }
    fn set_h(&mut self, value: u8) {
        self.HL = (self.HL & 0xff00) & value as u16
    }
    fn set_l(&mut self, value: u8) {
        self.HL = (self.HL & 0x00ff) & value as u16
    }

    fn set_fz(&mut self, z: bool) {
        self.AF = self.AF & 0x80 | (z as u16 & 1 << 7);
    }
    fn set_fn(&mut self, n: bool) {
        self.AF = self.AF & 0x40 | (n as u16 & 1 << 6);
    }
    fn set_fh(&mut self, h: bool) {
        self.AF = self.AF & 0x20 | (h as u16 & 1 << 5);
    }
    fn set_fc(&mut self, c: bool) {
        self.AF = self.AF & 0x10 | (c as u16 & 1 << 4);
    }

    fn set_clocks(&mut self, m: u8, t: u8) {
        self.clock_m = m;
        self.clock_t = t;
    }

    fn fetch_decode(&mut self, mem: &mut mem::Memory) {
        let opcode = mem.read_byte(self.PC);

        let (opcode_type, r) = match opcode {
            0xcb => (Opcode::CB, self.execute_cb(opcode, mem)),
            _ => (Opcode::Normal, self.execute(opcode, mem)),
        };

        self.handle_execute(opcode_type, r)
    }

    fn handle_execute(&mut self, opcode_type: Opcode, result: Instruction) {
        match result {
            Instruction::Info(length, clocks, _description) => {
                self.set_clocks(0, clocks);
                let _ = self.PC.wrapping_add(length);
            }
            Instruction::Invalid(opcode) => println!("invalid upcode {opcode} for {opcode_type}"),
            Instruction::Unimplemented(opcode) => println!("unimplemented opcode {opcode}"),
        }
    }
}
