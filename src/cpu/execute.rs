use crate::mem;
use helpers::Instruction;

use super::{helpers, Cpu, Register};

impl Cpu {
    pub fn execute(&mut self, opcode: u8, mem: &mut mem::Memory) -> Instruction {
        match opcode {
            0x00 => Instruction::Info(1, 4, "NOOP"),
            0x01 => {
                self.BC = self.get_nn(mem);
                Instruction::Info(3, 12, "LD BC,d16")
            }
            0x02 => {
                mem.write_byte(self.BC, self.get_a());
                Instruction::Info(1, 8, "LD (BC),A")
            }
            0x03 => {
                let _ = self.BC.wrapping_add(1);
                Instruction::Info(1, 8, "INC BC")
            }
            0x04 => {
                self.inc_register(Register::B);
                Instruction::Info(1, 4, "INC B")
            }
            0x05 => {
                self.dec_register(Register::B);
                Instruction::Info(1, 4, "DEC B")
            }
            0x06 => {
                self.set_b(self.get_n(mem));
                Instruction::Info(2, 8, "LD B,d8")
            }
            0x07 => Instruction::Unimplemented(opcode), //RCLA
            0x08 => {
                let addr = self.get_nn(mem);
                let val = mem.read_word(addr);
                mem.write_word(val, self.SP);
                Instruction::Info(2, 12, "LD (a16),SP")
            }
            0x09 => {
                self.HL = self.add_16(self.HL, self.BC);
                Instruction::Info(1, 8, "ADD HL,BC")
            }
            0x0a => {
                self.set_a(mem.read_byte(self.BC));
                Instruction::Info(1, 8, "LD A,(BC)")
            }
            0x0b => {
                let _ = self.BC.wrapping_sub(1);
                Instruction::Info(1, 8, "DEC BC")
            }
            0x0c => {
                self.inc_register(Register::C);
                Instruction::Info(1, 4, "INC C")
            }
            0x0d => {
                self.dec_register(Register::C);
                Instruction::Info(1, 4, "DEC C")
            }
            0x0e => {
                self.set_d(self.get_n(mem));
                Instruction::Info(2, 8, "LD C,d8")
            }
            0x0f => Instruction::Unimplemented(opcode), //RRCA
            0x10 => Instruction::Info(2, 4, "STOP"),    // TODO: figure out what stop does
            0x11 => {
                self.DE = self.get_nn(mem);
                Instruction::Info(3, 12, "LD DE,d16")
            }
            0x12 => {
                mem.write_byte(self.DE, self.get_a());
                Instruction::Info(1, 8, "LD (DE),A")
            }
            0x13 => {
                self.DE = self.DE.wrapping_add(1);
                Instruction::Info(1, 8, "INC DE")
            }
            0x14 => {
                self.inc_register(Register::D);
                Instruction::Info(1, 4, "INC D")
            }
            0x15 => {
                self.dec_register(Register::D);
                Instruction::Info(1, 4, "DEC D")
            }
            0x16 => {
                self.set_d(self.get_n(mem));
                Instruction::Info(2, 8, "LD D,d8")
            }
            0x17 => Instruction::Unimplemented(opcode), // RLA
            0x18 => Instruction::Unimplemented(opcode), //JR r8
            0x19 => {
                self.HL = self.add_16(self.HL, self.DE);
                Instruction::Info(1, 8, "ADD HL,DE")
            }
            0x1a => Instruction::Unimplemented(opcode),
            0x1b => Instruction::Unimplemented(opcode),
            0x1c => {
                self.inc_register(Register::E);
                Instruction::Info(1, 4, "INC E")
            }
            0x1d => {
                self.dec_register(Register::E);
                Instruction::Info(1, 4, "DEC E")
            }
            0x1e => {
                self.set_e(self.get_n(mem));
                Instruction::Info(2, 8, "LD E,d8")
            }
            0x1f => Instruction::Unimplemented(opcode), // RRA
            0x20 => Instruction::Unimplemented(opcode), // JR NC,r8
            0x21 => {
                self.HL = self.get_nn(mem);
                Instruction::Info(3, 12, "LD HL,d16")
            }
            0x22 => {
                mem.write_byte(self.HL, self.get_a());
                self.HL += 1;
                Instruction::Info(1, 8, "LD (HL+),A")
            }
            0x23 => Instruction::Unimplemented(opcode),
            0x24 => {
                self.inc_register(Register::H);
                Instruction::Info(1, 4, "INC H")
            }
            0x25 => {
                self.dec_register(Register::H);
                Instruction::Info(1, 4, "DEC H")
            }
            0x26 => {
                self.set_h(self.get_n(mem));
                Instruction::Info(2, 8, "LD H,d8")
            }
            0x27 => Instruction::Unimplemented(opcode),
            0x28 => Instruction::Unimplemented(opcode),
            0x29 => Instruction::Unimplemented(opcode),
            0x2a => Instruction::Unimplemented(opcode),
            0x2b => Instruction::Unimplemented(opcode),
            0x2c => {
                self.inc_register(Register::L);
                Instruction::Info(1, 4, "INC L")
            }
            0x2d => {
                self.dec_register(Register::L);
                Instruction::Info(1, 4, "DEC L")
            }
            0x2e => {
                self.set_l(self.get_n(mem));
                Instruction::Info(2, 8, "LD L,d8")
            }
            0x2f => {
                self.set_a(!self.get_a());
                self.set_fn(true);
                self.set_fh(true);
                Instruction::Info(1, 4, "CPL")
            }
            0x30 => Instruction::Unimplemented(opcode), // JR NC,r8
            0x31 => {
                self.SP = self.get_nn(mem);
                Instruction::Info(3, 12, "LD SP,d16")
            }
            0x32 => {
                mem.write_byte(self.HL, self.get_a());
                self.HL -= 1;
                Instruction::Info(1, 8, "LD (HL+),A")
            }
            0x33 => {
                self.SP = self.SP.wrapping_add(1);
                Instruction::Info(1, 8, "INC SP")
            }
            0x34 => {
                let val = self.HL;
                let result =self.add_16(mem.read_word(val), 1);
                mem.write_word(val, result);
                Instruction::Info(2, 12,"INC (HL)" )
            }
            0x35 => {
                let val = self.HL;
                let result =self.sub_16(mem.read_word(val), 1);
                mem.write_word(val, result);
                Instruction::Info(2, 12,"DEC (HL)" )
            }
            0x36 => Instruction::Unimplemented(opcode),
            0x37 => Instruction::Unimplemented(opcode),
            0x38 => Instruction::Unimplemented(opcode),
            0x39 => Instruction::Unimplemented(opcode),
            0x3a => Instruction::Unimplemented(opcode),
            0x3b => Instruction::Unimplemented(opcode),
            0x3c => {
                self.inc_register(Register::A);
                Instruction::Info(1, 4, "INC A")
            }
            0x3d => {
                self.dec_register(Register::A);
                Instruction::Info(1, 4, "DEC A")
            }
            0x3e => {
                self.set_a(self.get_n(mem));
                Instruction::Info(2, 8, "LD A,d8")
            }
            0x3f => Instruction::Unimplemented(opcode),
            0x40 => Instruction::Unimplemented(opcode),
            0x41 => Instruction::Unimplemented(opcode),
            0x42 => Instruction::Unimplemented(opcode),
            0x43 => Instruction::Unimplemented(opcode),
            0x44 => Instruction::Unimplemented(opcode),
            0x45 => Instruction::Unimplemented(opcode),
            0x46 => Instruction::Unimplemented(opcode),
            0x47 => Instruction::Unimplemented(opcode),
            0x48 => Instruction::Unimplemented(opcode),
            0x49 => Instruction::Unimplemented(opcode),
            0x4a => Instruction::Unimplemented(opcode),
            0x4b => Instruction::Unimplemented(opcode),
            0x4c => Instruction::Unimplemented(opcode),
            0x4d => Instruction::Unimplemented(opcode),
            0x4e => Instruction::Unimplemented(opcode),
            0x4f => Instruction::Unimplemented(opcode),
            0x50 => Instruction::Unimplemented(opcode),
            0x51 => Instruction::Unimplemented(opcode),
            0x52 => Instruction::Unimplemented(opcode),
            0x53 => Instruction::Unimplemented(opcode),
            0x54 => Instruction::Unimplemented(opcode),
            0x55 => Instruction::Unimplemented(opcode),
            0x56 => Instruction::Unimplemented(opcode),
            0x57 => Instruction::Unimplemented(opcode),
            0x58 => Instruction::Unimplemented(opcode),
            0x59 => Instruction::Unimplemented(opcode),
            0x5a => Instruction::Unimplemented(opcode),
            0x5b => Instruction::Unimplemented(opcode),
            0x5c => Instruction::Unimplemented(opcode),
            0x5d => Instruction::Unimplemented(opcode),
            0x5e => Instruction::Unimplemented(opcode),
            0x5f => Instruction::Unimplemented(opcode),
            0x60 => Instruction::Unimplemented(opcode),
            0x61 => Instruction::Unimplemented(opcode),
            0x62 => Instruction::Unimplemented(opcode),
            0x63 => Instruction::Unimplemented(opcode),
            0x64 => Instruction::Unimplemented(opcode),
            0x65 => Instruction::Unimplemented(opcode),
            0x66 => Instruction::Unimplemented(opcode),
            0x67 => Instruction::Unimplemented(opcode),
            0x68 => Instruction::Unimplemented(opcode),
            0x69 => Instruction::Unimplemented(opcode),
            0x6a => Instruction::Unimplemented(opcode),
            0x6b => Instruction::Unimplemented(opcode),
            0x6c => Instruction::Unimplemented(opcode),
            0x6d => Instruction::Unimplemented(opcode),
            0x6e => Instruction::Unimplemented(opcode),
            0x6f => Instruction::Unimplemented(opcode),
            0x70 => Instruction::Unimplemented(opcode),
            0x71 => Instruction::Unimplemented(opcode),
            0x72 => Instruction::Unimplemented(opcode),
            0x73 => Instruction::Unimplemented(opcode),
            0x74 => Instruction::Unimplemented(opcode),
            0x75 => Instruction::Unimplemented(opcode),
            0x76 => Instruction::Unimplemented(opcode),
            0x77 => Instruction::Unimplemented(opcode),
            0x78 => Instruction::Unimplemented(opcode),
            0x79 => Instruction::Unimplemented(opcode),
            0x7a => Instruction::Unimplemented(opcode),
            0x7b => Instruction::Unimplemented(opcode),
            0x7c => Instruction::Unimplemented(opcode),
            0x7d => Instruction::Unimplemented(opcode),
            0x7e => Instruction::Unimplemented(opcode),
            0x7f => Instruction::Unimplemented(opcode),
            0x80 => Instruction::Unimplemented(opcode),
            0x81 => Instruction::Unimplemented(opcode),
            0x82 => Instruction::Unimplemented(opcode),
            0x83 => Instruction::Unimplemented(opcode),
            0x84 => Instruction::Unimplemented(opcode),
            0x85 => Instruction::Unimplemented(opcode),
            0x86 => Instruction::Unimplemented(opcode),
            0x87 => Instruction::Unimplemented(opcode),
            0x88 => Instruction::Unimplemented(opcode),
            0x89 => Instruction::Unimplemented(opcode),
            0x8a => Instruction::Unimplemented(opcode),
            0x8b => Instruction::Unimplemented(opcode),
            0x8c => Instruction::Unimplemented(opcode),
            0x8d => Instruction::Unimplemented(opcode),
            0x8e => Instruction::Unimplemented(opcode),
            0x8f => Instruction::Unimplemented(opcode),
            0x90 => Instruction::Unimplemented(opcode),
            0x91 => Instruction::Unimplemented(opcode),
            0x92 => Instruction::Unimplemented(opcode),
            0x93 => Instruction::Unimplemented(opcode),
            0x94 => Instruction::Unimplemented(opcode),
            0x95 => Instruction::Unimplemented(opcode),
            0x96 => Instruction::Unimplemented(opcode),
            0x97 => Instruction::Unimplemented(opcode),
            0x98 => Instruction::Unimplemented(opcode),
            0x99 => Instruction::Unimplemented(opcode),
            0x9a => Instruction::Unimplemented(opcode),
            0x9b => Instruction::Unimplemented(opcode),
            0x9c => Instruction::Unimplemented(opcode),
            0x9d => Instruction::Unimplemented(opcode),
            0x9e => Instruction::Unimplemented(opcode),
            0x9f => Instruction::Unimplemented(opcode),
            0xa0 => Instruction::Unimplemented(opcode),
            0xa1 => Instruction::Unimplemented(opcode),
            0xa2 => Instruction::Unimplemented(opcode),
            0xa3 => Instruction::Unimplemented(opcode),
            0xa4 => Instruction::Unimplemented(opcode),
            0xa5 => Instruction::Unimplemented(opcode),
            0xa6 => Instruction::Unimplemented(opcode),
            0xa7 => Instruction::Unimplemented(opcode),
            0xa8 => Instruction::Unimplemented(opcode),
            0xa9 => Instruction::Unimplemented(opcode),
            0xaa => Instruction::Unimplemented(opcode),
            0xab => Instruction::Unimplemented(opcode),
            0xac => Instruction::Unimplemented(opcode),
            0xad => Instruction::Unimplemented(opcode),
            0xae => Instruction::Unimplemented(opcode),
            0xaf => Instruction::Unimplemented(opcode),
            0xb0 => Instruction::Unimplemented(opcode),
            0xb1 => Instruction::Unimplemented(opcode),
            0xb2 => Instruction::Unimplemented(opcode),
            0xb3 => Instruction::Unimplemented(opcode),
            0xb4 => Instruction::Unimplemented(opcode),
            0xb5 => Instruction::Unimplemented(opcode),
            0xb6 => Instruction::Unimplemented(opcode),
            0xb7 => Instruction::Unimplemented(opcode),
            0xb8 => Instruction::Unimplemented(opcode),
            0xb9 => Instruction::Unimplemented(opcode),
            0xba => Instruction::Unimplemented(opcode),
            0xbb => Instruction::Unimplemented(opcode),
            0xbc => Instruction::Unimplemented(opcode),
            0xbd => Instruction::Unimplemented(opcode),
            0xbe => Instruction::Unimplemented(opcode),
            0xbf => Instruction::Unimplemented(opcode),
            0xc0 => Instruction::Unimplemented(opcode),
            0xc1 => Instruction::Unimplemented(opcode),
            0xc2 => Instruction::Unimplemented(opcode),
            0xc3 => Instruction::Unimplemented(opcode),
            0xc4 => Instruction::Unimplemented(opcode),
            0xc5 => Instruction::Unimplemented(opcode),
            0xc6 => Instruction::Unimplemented(opcode),
            0xc7 => Instruction::Unimplemented(opcode),
            0xc8 => Instruction::Unimplemented(opcode),
            0xc9 => Instruction::Unimplemented(opcode),
            0xca => Instruction::Unimplemented(opcode),
            0xcb => Instruction::Unimplemented(opcode),
            0xcc => Instruction::Unimplemented(opcode),
            0xcd => Instruction::Unimplemented(opcode),
            0xce => Instruction::Unimplemented(opcode),
            0xcf => Instruction::Unimplemented(opcode),
            0xd0 => Instruction::Unimplemented(opcode),
            0xd1 => Instruction::Unimplemented(opcode),
            0xd2 => Instruction::Unimplemented(opcode),
            0xd3 => Instruction::Unimplemented(opcode),
            0xd4 => Instruction::Unimplemented(opcode),
            0xd5 => Instruction::Unimplemented(opcode),
            0xd6 => Instruction::Unimplemented(opcode),
            0xd7 => Instruction::Unimplemented(opcode),
            0xd8 => Instruction::Unimplemented(opcode),
            0xd9 => Instruction::Unimplemented(opcode),
            0xda => Instruction::Unimplemented(opcode),
            0xdb => Instruction::Unimplemented(opcode),
            0xdc => Instruction::Unimplemented(opcode),
            0xdd => Instruction::Unimplemented(opcode),
            0xde => Instruction::Unimplemented(opcode),
            0xdf => Instruction::Unimplemented(opcode),
            0xe0 => Instruction::Unimplemented(opcode),
            0xe1 => Instruction::Unimplemented(opcode),
            0xe2 => Instruction::Unimplemented(opcode),
            0xe3 => Instruction::Unimplemented(opcode),
            0xe4 => Instruction::Unimplemented(opcode),
            0xe5 => Instruction::Unimplemented(opcode),
            0xe6 => Instruction::Unimplemented(opcode),
            0xe7 => Instruction::Unimplemented(opcode),
            0xe8 => Instruction::Unimplemented(opcode),
            0xe9 => Instruction::Unimplemented(opcode),
            0xea => Instruction::Unimplemented(opcode),
            0xeb => Instruction::Unimplemented(opcode),
            0xec => Instruction::Unimplemented(opcode),
            0xed => Instruction::Unimplemented(opcode),
            0xee => Instruction::Unimplemented(opcode),
            0xef => Instruction::Unimplemented(opcode),
            0xf0 => Instruction::Unimplemented(opcode),
            0xf1 => Instruction::Unimplemented(opcode),
            0xf2 => Instruction::Unimplemented(opcode),
            0xf3 => Instruction::Unimplemented(opcode),
            0xf4 => Instruction::Unimplemented(opcode),
            0xf5 => Instruction::Unimplemented(opcode),
            0xf6 => Instruction::Unimplemented(opcode),
            0xf7 => Instruction::Unimplemented(opcode),
            0xf8 => Instruction::Unimplemented(opcode),
            0xf9 => Instruction::Unimplemented(opcode),
            0xfa => Instruction::Unimplemented(opcode),
            0xfb => Instruction::Unimplemented(opcode),
            0xfc => Instruction::Unimplemented(opcode),
            0xfd => Instruction::Unimplemented(opcode),
            0xfe => Instruction::Unimplemented(opcode),
            _ => Instruction::Invalid(opcode),
        }
    }
}
