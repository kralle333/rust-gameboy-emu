use std::ops::{Shl, Shr};

use crate::memory::{self, MemoryType};
use helpers::Instruction;

use super::{helpers, Cpu, Flag, Register};

impl Cpu {
    pub fn execute(&mut self, opcode: u8, mem: &mut memory::Memory) -> Instruction {
        match opcode {
            0x00 => Instruction::Ok(opcode, 1, 4, "NOOP"),
            0x01 => {
                self.BC = self.get_nn(mem);
                Instruction::Ok(opcode, 3, 12, "LD BC,d16")
            }
            0x02 => {
                mem.write_byte(self.BC, self.get_a());
                Instruction::Ok(opcode, 1, 8, "LD (BC),A")
            }
            0x03 => {
                let _ = self.BC.wrapping_add(1);
                Instruction::Ok(opcode, 1, 8, "INC BC")
            }
            0x04 => {
                self.inc_register(Register::B);
                Instruction::Ok(opcode, 1, 4, "INC B")
            }
            0x05 => {
                self.dec_register(Register::B);
                Instruction::Ok(opcode, 1, 4, "DEC B")
            }
            0x06 => {
                self.set_b(self.get_n(mem));
                Instruction::Ok(opcode, 2, 8, "LD B,d8")
            }
            0x07 => {
                let val = self.get_a();
                self.set_a(val.rotate_left(1));
                self.set_flag(Flag::C, (0x80 & val) != 0);
                Instruction::Ok(opcode, 1, 4, "RLCA")
            }
            0x08 => {
                let addr = self.get_nn(mem);
                let val = mem.read_word(addr);
                mem.write_word(val, self.SP);
                Instruction::Ok(opcode, 2, 12, "LD (a16),SP")
            }
            0x09 => {
                self.HL = self.add_16(self.HL, self.BC);
                Instruction::Ok(opcode, 1, 8, "ADD HL,BC")
            }
            0x0a => {
                self.set_a(mem.read_byte(self.BC));
                Instruction::Ok(opcode, 1, 8, "LD A,(BC)")
            }
            0x0b => {
                let _ = self.BC.wrapping_sub(1);
                Instruction::Ok(opcode, 1, 8, "DEC BC")
            }
            0x0c => {
                self.inc_register(Register::C);
                Instruction::Ok(opcode, 1, 4, "INC C")
            }
            0x0d => {
                self.dec_register(Register::C);
                Instruction::Ok(opcode, 1, 4, "DEC C")
            }
            0x0e => {
                self.set_d(self.get_n(mem));
                Instruction::Ok(opcode, 2, 8, "LD C,d8")
            }
            0x0f => {
                let val = self.get_a();
                self.set_a(val.rotate_right(1));
                self.reset_all_flags();
                self.set_flag(Flag::C, (val & 1) == 1);
                Instruction::Ok(opcode, 1, 4, "RRCA")
            }
            0x10 => Instruction::Ok(opcode, 2, 4, "STOP"), // TODO: figure out what stop does
            0x11 => {
                self.DE = self.get_nn(mem);
                Instruction::Ok(opcode, 3, 12, "LD DE,d16")
            }
            0x12 => {
                mem.write_byte(self.DE, self.get_a());
                Instruction::Ok(opcode, 1, 8, "LD (DE),A")
            }
            0x13 => {
                self.DE = self.DE.wrapping_add(1);
                Instruction::Ok(opcode, 1, 8, "INC DE")
            }
            0x14 => {
                self.inc_register(Register::D);
                Instruction::Ok(opcode, 1, 4, "INC D")
            }
            0x15 => {
                self.dec_register(Register::D);
                Instruction::Ok(opcode, 1, 4, "DEC D")
            }
            0x16 => {
                self.set_d(self.get_n(mem));
                Instruction::Ok(opcode, 2, 8, "LD D,d8")
            }
            0x17 => {
                let a = self.get_a();
                self.set_a(a.shl(1) | (self.get_flag(Flag::C) as u8));
                self.reset_all_flags();
                self.set_flag(Flag::C, (0x80 & a) != 0);
                Instruction::Ok(opcode, 1, 4, "RLA")
            }
            0x18 => {
                self.PC = self.PC.wrapping_add(self.get_n(mem) as u16);
                Instruction::Ok(opcode, 0, 3, "JR r8")
            }
            0x19 => {
                self.HL = self.add_16(self.HL, self.DE);
                Instruction::Ok(opcode, 1, 8, "ADD HL,DE")
            }
            0x1a => {
                self.set_a(mem.read_byte(self.DE));
                Instruction::Ok(opcode, 1, 2, "LD A, (DE)")
            }
            0x1b => {
                self.DE = self.sub_16(self.DE, 1);
                Instruction::Ok(opcode, 1, 2, "DEC DE")
            }
            0x1c => {
                self.inc_register(Register::E);
                Instruction::Ok(opcode, 1, 4, "INC E")
            }
            0x1d => {
                self.dec_register(Register::E);
                Instruction::Ok(opcode, 1, 4, "DEC E")
            }
            0x1e => {
                self.set_e(self.get_n(mem));
                Instruction::Ok(opcode, 2, 8, "LD E,d8")
            }
            0x1f => {
                let a = self.get_a();
                self.set_a(a.shr(1) | (self.get_flag(Flag::C) as u8).shl(7));
                self.reset_all_flags();
                self.set_flag(Flag::C, (1 & a) != 0);
                Instruction::Ok(opcode, 1, 1, "RRA")
            }
            0x20 => {
                if !self.get_flag(Flag::C) {
                    self.PC = self.PC.wrapping_add(self.get_n(mem) as u16);
                    return Instruction::Ok(opcode, 0, 3, "JR NC,r8");
                }
                Instruction::Ok(opcode, 2, 2, "JR NC,r8")
            }
            0x21 => {
                self.HL = self.get_nn(mem);
                Instruction::Ok(opcode, 3, 12, "LD HL,d16")
            }
            0x22 => {
                mem.write_byte(self.HL, self.get_a());
                self.HL = self.HL.wrapping_add(1);
                Instruction::Ok(opcode, 1, 8, "LD (HL+),A")
            }
            0x23 => {
                self.HL = self.HL.wrapping_add(1);
                Instruction::Ok(opcode, 1, 8, "INC HL")
            }
            0x24 => {
                self.inc_register(Register::H);
                Instruction::Ok(opcode, 1, 4, "INC H")
            }
            0x25 => {
                self.dec_register(Register::H);
                Instruction::Ok(opcode, 1, 4, "DEC H")
            }
            0x26 => {
                self.set_h(self.get_n(mem));
                Instruction::Ok(opcode, 2, 8, "LD H,d8")
            }
            0x27 => {
                let a = self.get_a();
                if a > 9 {}
                //TODO: Just take solution somewhere

                Instruction::Ok(opcode, 1, 1, "DAA")
            }
            0x28 => {
                if self.get_flag(Flag::Z) {
                    self.PC = self.PC.wrapping_add(self.get_n(mem) as u16);
                    return Instruction::Ok(opcode, 0, 3, "JR Z,r8");
                }
                Instruction::Ok(opcode, 2, 2, "JR Z,r8")
            }
            0x29 => {
                self.HL = self.add_16(self.HL, self.HL);
                Instruction::Ok(opcode, 1, 8, "ADD HL,HL")
            }
            0x2a => {
                self.set_a(mem.read_byte(self.HL));
                self.HL = self.HL.wrapping_add(1);
                Instruction::Ok(opcode, 1, 8, "LD A,(HL+)")
            }
            0x2b => {
                self.HL = self.HL.wrapping_sub(1);
                Instruction::Ok(opcode, 1, 8, "DEC HL")
            }

            0x2c => {
                self.inc_register(Register::L);
                Instruction::Ok(opcode, 1, 4, "INC L")
            }
            0x2d => {
                self.dec_register(Register::L);
                Instruction::Ok(opcode, 1, 4, "DEC L")
            }
            0x2e => {
                self.set_l(self.get_n(mem));
                Instruction::Ok(opcode, 2, 8, "LD L,d8")
            }
            0x2f => {
                self.set_a(!self.get_a());
                self.set_flag(Flag::N, true);
                self.set_flag(Flag::H, true);
                Instruction::Ok(opcode, 1, 4, "CPL")
            }
            0x30 => {
                if !self.get_flag(Flag::C) {
                    self.PC = self.PC.wrapping_add(self.get_n(mem) as u16);
                    return Instruction::Ok(opcode, 0, 12, "JR NC,r8");
                }
                Instruction::Ok(opcode, 2, 8, "JR NC,r8")
            }
            0x31 => {
                self.SP = self.get_nn(mem);
                Instruction::Ok(opcode, 3, 12, "LD SP,d16")
            }
            0x32 => {
                mem.write_byte(self.HL, self.get_a());
                self.HL -= 1;
                Instruction::Ok(opcode, 1, 8, "LD (HL+),A")
            }
            0x33 => {
                self.SP = self.SP.wrapping_add(1);
                Instruction::Ok(opcode, 1, 8, "INC SP")
            }
            0x34 => {
                let val = self.HL;
                let result = self.add_16(mem.read_word(val), 1);
                mem.write_word(val, result);
                Instruction::Ok(opcode, 2, 12, "INC (HL)")
            }
            0x35 => {
                let val = self.HL;
                let result = self.sub_16(mem.read_word(val), 1);
                mem.write_word(val, result);
                Instruction::Ok(opcode, 2, 12, "DEC (HL)")
            }
            0x36 => {
                let val = self.get_n(mem);
                mem.write_byte(self.HL, val);
                Instruction::Ok(opcode, 2, 12, "LD (HL),d8")
            }
            0x37 => {
                self.set_flag(Flag::C, true);
                Instruction::Ok(opcode, 1, 4, "SCF")
            }
            0x38 => {
                if self.get_flag(Flag::C) {
                    self.PC = self.PC.wrapping_add(self.get_n(mem) as u16);
                    return Instruction::Ok(opcode, 0, 12, "JR C,r8");
                }
                Instruction::Ok(opcode, 2, 8, "JR C,r8")
            }
            0x39 => {
                self.HL = self.add_16(self.HL, self.SP);
                Instruction::Ok(opcode, 1, 8, "ADD HL,SP")
            }
            0x3a => {
                self.set_a(mem.read_byte(self.HL));
                self.HL = self.HL.wrapping_sub(1);
                Instruction::Ok(opcode, 1, 8, "LD A,(HL-)")
            }
            0x3b => {
                self.SP = self.sub_16(self.SP, 1);
                Instruction::Ok(opcode, 1, 2, "DEC SP")
            }
            0x3c => {
                self.inc_register(Register::A);
                Instruction::Ok(opcode, 1, 4, "INC A")
            }
            0x3d => {
                self.dec_register(Register::A);
                Instruction::Ok(opcode, 1, 4, "DEC A")
            }
            0x3e => {
                self.set_a(self.get_n(mem));
                Instruction::Ok(opcode, 2, 8, "LD A,d8")
            }
            0x3f => {
                self.set_flag(Flag::N, false);
                self.set_flag(Flag::H, false);
                self.set_flag(Flag::C, !self.get_flag(Flag::C));
                Instruction::Ok(opcode, 1, 1, "CCF")
            }
            0x40 => {
                self.set_b(self.get_b());
                Instruction::Ok(opcode, 1, 4, "LD B, B")
            }
            0x41 => {
                self.set_b(self.get_c());
                Instruction::Ok(opcode, 1, 4, "LD B, C")
            }
            0x42 => {
                self.set_b(self.get_d());
                Instruction::Ok(opcode, 1, 4, "LD B, D")
            }
            0x43 => {
                self.set_b(self.get_e());
                Instruction::Ok(opcode, 1, 4, "LD B, E")
            }
            0x44 => {
                self.set_b(self.get_h());
                Instruction::Ok(opcode, 1, 4, "LD B, H")
            }
            0x45 => {
                self.set_b(self.get_l());
                Instruction::Ok(opcode, 1, 4, "LD B, L")
            }
            0x46 => {
                self.set_b(mem.read_byte(self.HL));
                Instruction::Ok(opcode, 1, 8, "LD B, (HL)")
            }
            0x47 => {
                self.set_b(self.get_a());
                Instruction::Ok(opcode, 1, 4, "LD B, A")
            }
            0x48 => {
                self.set_c(self.get_b());
                Instruction::Ok(opcode, 1, 4, "LD C, B")
            }
            0x49 => {
                self.set_c(self.get_c());
                Instruction::Ok(opcode, 1, 4, "LD C, C")
            }
            0x4a => {
                self.set_c(self.get_d());
                Instruction::Ok(opcode, 1, 4, "LD C, D")
            }
            0x4b => {
                self.set_c(self.get_e());
                Instruction::Ok(opcode, 1, 4, "LD C, E")
            }
            0x4c => {
                self.set_c(self.get_h());
                Instruction::Ok(opcode, 1, 4, "LD C, H")
            }
            0x4d => {
                self.set_c(self.get_l());
                Instruction::Ok(opcode, 1, 4, "LD C, L")
            }
            0x4e => {
                self.set_c(mem.read_byte(self.HL));
                Instruction::Ok(opcode, 1, 8, "LD C, (HL)")
            }
            0x4f => {
                self.set_c(self.get_a());
                Instruction::Ok(opcode, 1, 4, "LD C, A")
            }
            0x50 => {
                self.set_d(self.get_b());
                Instruction::Ok(opcode, 1, 4, "LD D, B")
            }
            0x51 => {
                self.set_d(self.get_c());
                Instruction::Ok(opcode, 1, 4, "LD D, C")
            }
            0x52 => {
                self.set_d(self.get_d());
                Instruction::Ok(opcode, 1, 4, "LD D, D")
            }
            0x53 => {
                self.set_d(self.get_e());
                Instruction::Ok(opcode, 1, 4, "LD D, E")
            }
            0x54 => {
                self.set_d(self.get_h());
                Instruction::Ok(opcode, 1, 4, "LD D, H")
            }
            0x55 => {
                self.set_d(self.get_l());
                Instruction::Ok(opcode, 1, 4, "LD D, L")
            }
            0x56 => {
                self.set_d(mem.read_byte(self.HL));
                Instruction::Ok(opcode, 1, 8, "LD D, (HL)")
            }
            0x57 => {
                self.set_d(self.get_a());
                Instruction::Ok(opcode, 1, 4, "LD D, A")
            }
            0x58 => {
                self.set_e(self.get_b());
                Instruction::Ok(opcode, 1, 4, "LD E, B")
            }
            0x59 => {
                self.set_e(self.get_c());
                Instruction::Ok(opcode, 1, 4, "LD E, C")
            }
            0x5a => {
                self.set_e(self.get_d());
                Instruction::Ok(opcode, 1, 4, "LD E, D")
            }
            0x5b => {
                self.set_e(self.get_e());
                Instruction::Ok(opcode, 1, 4, "LD E, E")
            }
            0x5c => {
                self.set_e(self.get_h());
                Instruction::Ok(opcode, 1, 4, "LD E, H")
            }
            0x5d => {
                self.set_e(self.get_l());
                Instruction::Ok(opcode, 1, 4, "LD E, L")
            }
            0x5e => {
                self.set_e(mem.read_byte(self.HL));
                Instruction::Ok(opcode, 1, 8, "LD E, (HL)")
            }
            0x5f => {
                self.set_e(self.get_a());
                Instruction::Ok(opcode, 1, 4, "LD E, A")
            }
            0x60 => {
                self.set_h(self.get_b());
                Instruction::Ok(opcode, 1, 4, "LD H, B")
            }
            0x61 => {
                self.set_h(self.get_c());
                Instruction::Ok(opcode, 1, 4, "LD H, C")
            }
            0x62 => {
                self.set_h(self.get_d());
                Instruction::Ok(opcode, 1, 4, "LD H, D")
            }
            0x63 => {
                self.set_h(self.get_e());
                Instruction::Ok(opcode, 1, 4, "LD H, E")
            }
            0x64 => {
                self.set_h(self.get_h());
                Instruction::Ok(opcode, 1, 4, "LD H, H")
            }
            0x65 => {
                self.set_h(self.get_l());
                Instruction::Ok(opcode, 1, 4, "LD H, L")
            }
            0x66 => {
                self.set_h(mem.read_byte(self.HL));
                Instruction::Ok(opcode, 1, 8, "LD H, (HL)")
            }
            0x67 => {
                self.set_h(self.get_a());
                Instruction::Ok(opcode, 1, 4, "LD H, A")
            }
            0x68 => {
                self.set_l(self.get_b());
                Instruction::Ok(opcode, 1, 4, "LD L, B")
            }
            0x69 => {
                self.set_l(self.get_c());
                Instruction::Ok(opcode, 1, 4, "LD L, C")
            }
            0x6a => {
                self.set_l(self.get_d());
                Instruction::Ok(opcode, 1, 4, "LD L, D")
            }
            0x6b => {
                self.set_l(self.get_e());
                Instruction::Ok(opcode, 1, 4, "LD L, E")
            }
            0x6c => {
                self.set_l(self.get_h());
                Instruction::Ok(opcode, 1, 4, "LD L, H")
            }
            0x6d => {
                self.set_l(self.get_l());
                Instruction::Ok(opcode, 1, 4, "LD L, L")
            }
            0x6e => {
                self.set_l(mem.read_byte(self.HL));
                Instruction::Ok(opcode, 1, 8, "LD L, (HL)")
            }
            0x6f => {
                self.set_l(self.get_a());
                Instruction::Ok(opcode, 1, 4, "LD L, A")
            }
            0x70 => {
                mem.write_byte(self.HL, self.get_b());
                Instruction::Ok(opcode, 1, 4, "LD (HL), B")
            }
            0x71 => {
                mem.write_byte(self.HL, self.get_c());
                Instruction::Ok(opcode, 1, 8, "LD (HL), C")
            }
            0x72 => {
                mem.write_byte(self.HL, self.get_d());
                Instruction::Ok(opcode, 1, 8, "LD (HL), D")
            }
            0x73 => {
                mem.write_byte(self.HL, self.get_e());
                Instruction::Ok(opcode, 1, 8, "LD (HL), E")
            }
            0x74 => {
                mem.write_byte(self.HL, self.get_h());
                Instruction::Ok(opcode, 1, 8, "LD (HL), H")
            }
            0x75 => {
                mem.write_byte(self.HL, self.get_l());
                Instruction::Ok(opcode, 1, 8, "LD (HL), L")
            }
            0x76 => {
                // TODO: Figure out HALT
                Instruction::Ok(opcode, 1, 4, "HALT")
            }
            0x77 => {
                mem.write_byte(self.HL, self.get_a());
                Instruction::Ok(opcode, 1, 8, "LD (HL), A")
            }
            0x78 => {
                self.set_a(self.get_b());
                Instruction::Ok(opcode, 1, 4, "LD A, B")
            }
            0x79 => {
                self.set_a(self.get_c());
                Instruction::Ok(opcode, 1, 4, "LD A, C")
            }
            0x7a => {
                self.set_a(self.get_d());
                Instruction::Ok(opcode, 1, 4, "LD A, D")
            }
            0x7b => {
                self.set_a(self.get_e());
                Instruction::Ok(opcode, 1, 4, "LD A, E")
            }
            0x7c => {
                self.set_a(self.get_h());
                Instruction::Ok(opcode, 1, 4, "LD A, H")
            }
            0x7d => {
                self.set_a(self.get_l());
                Instruction::Ok(opcode, 1, 4, "LD A, L")
            }
            0x7e => {
                self.set_a(mem.read_byte(self.HL));
                Instruction::Ok(opcode, 1, 8, "LD A, (HL)")
            }
            0x7f => {
                self.set_a(self.get_a());
                Instruction::Ok(opcode, 1, 4, "LD A, A")
            }
            0x80 => {
                self.add_a(self.get_b());
                Instruction::Ok(opcode, 1, 4, "ADD A,B")
            }
            0x81 => {
                self.add_a(self.get_c());
                Instruction::Ok(opcode, 1, 4, "ADD A, C")
            }
            0x82 => {
                self.add_a(self.get_d());
                Instruction::Ok(opcode, 1, 4, "ADD A, D")
            }
            0x83 => {
                self.add_a(self.get_e());
                Instruction::Ok(opcode, 1, 4, "ADD A, E")
            }
            0x84 => {
                self.add_a(self.get_h());
                Instruction::Ok(opcode, 1, 4, "ADD A, H")
            }
            0x85 => {
                self.add_a(self.get_l());
                Instruction::Ok(opcode, 1, 4, "ADD A, L")
            }
            0x86 => {
                self.add_a(mem.read_byte(self.HL));
                Instruction::Ok(opcode, 1, 8, "ADD A, (HL)")
            }
            0x87 => {
                self.add_a(self.get_a());
                Instruction::Ok(opcode, 1, 4, "ADD A, A")
            }
            0x88 => {
                self.adc_a(self.get_b());
                Instruction::Ok(opcode, 1, 4, "ADD A, B")
            }
            0x89 => {
                self.adc_a(self.get_c());
                Instruction::Ok(opcode, 1, 4, "ADD A, C")
            }
            0x8a => {
                self.adc_a(self.get_d());
                Instruction::Ok(opcode, 1, 4, "ADD A, D")
            }
            0x8b => {
                self.adc_a(self.get_e());
                Instruction::Ok(opcode, 1, 4, "ADD A, E")
            }
            0x8c => {
                self.adc_a(self.get_h());
                Instruction::Ok(opcode, 1, 4, "ADD A, H")
            }
            0x8d => {
                self.adc_a(self.get_l());
                Instruction::Ok(opcode, 1, 4, "ADD A, L")
            }
            0x8e => {
                self.adc_a(mem.read_byte(self.HL));
                Instruction::Ok(opcode, 1, 8, "ADD A, (HL)")
            }
            0x8f => {
                self.adc_a(self.get_a());
                Instruction::Ok(opcode, 1, 4, "ADD A, A")
            }
            0x90 => {
                self.sub_a(self.get_b());
                Instruction::Ok(opcode, 1, 4, "SUB B")
            }
            0x91 => {
                self.sub_a(self.get_c());
                Instruction::Ok(opcode, 1, 4, "SUB C")
            }
            0x92 => {
                self.sub_a(self.get_d());
                Instruction::Ok(opcode, 1, 4, "SUB D")
            }
            0x93 => {
                self.sub_a(self.get_e());
                Instruction::Ok(opcode, 1, 4, "SUB E")
            }
            0x94 => {
                self.sub_a(self.get_h());
                Instruction::Ok(opcode, 1, 4, "SUB H")
            }
            0x95 => {
                self.sub_a(self.get_l());
                Instruction::Ok(opcode, 1, 4, "SUB L")
            }
            0x96 => {
                self.sub_a(mem.read_byte(self.HL));
                Instruction::Ok(opcode, 1, 8, "SUB (HL)")
            }
            0x97 => {
                self.sub_a(self.get_a());
                Instruction::Ok(opcode, 1, 4, "SUB A")
            }
            0x98 => {
                self.sbc_a(self.get_b());
                Instruction::Ok(opcode, 1, 4, "SBC A, B")
            }
            0x99 => {
                self.sbc_a(self.get_c());
                Instruction::Ok(opcode, 1, 4, "SBC A, C")
            }
            0x9a => {
                self.sbc_a(self.get_d());
                Instruction::Ok(opcode, 1, 4, "SBC A, D")
            }
            0x9b => {
                self.sbc_a(self.get_e());
                Instruction::Ok(opcode, 1, 4, "SBC A, E")
            }
            0x9c => {
                self.sbc_a(self.get_h());
                Instruction::Ok(opcode, 1, 4, "SBC A, H")
            }
            0x9d => {
                self.sbc_a(self.get_l());
                Instruction::Ok(opcode, 1, 4, "SBC A, L")
            }
            0x9e => {
                self.sbc_a(mem.read_byte(self.HL));
                Instruction::Ok(opcode, 1, 8, "SBC A, (HL)")
            }
            0x9f => {
                self.sbc_a(self.get_a());
                Instruction::Ok(opcode, 1, 4, "SBC A, A")
            }
            0xa0 => {
                self.and_a(self.get_b());
                Instruction::Ok(opcode, 1, 4, "AND B")
            }
            0xa1 => {
                self.and_a(self.get_c());
                Instruction::Ok(opcode, 1, 4, "AND C")
            }
            0xa2 => {
                self.and_a(self.get_d());
                Instruction::Ok(opcode, 1, 4, "AND D")
            }
            0xa3 => {
                self.and_a(self.get_e());
                Instruction::Ok(opcode, 1, 4, "AND E")
            }
            0xa4 => {
                self.and_a(self.get_h());
                Instruction::Ok(opcode, 1, 4, "AND H")
            }
            0xa5 => {
                self.and_a(self.get_l());
                Instruction::Ok(opcode, 1, 4, "AND L")
            }
            0xa6 => {
                self.and_a(mem.read_byte(self.HL));
                Instruction::Ok(opcode, 1, 2, "AND (HL)")
            }
            0xa7 => {
                self.and_a(self.get_a());
                Instruction::Ok(opcode, 1, 4, "AND A")
            }
            0xa8 => {
                self.xor_a(self.get_b());
                Instruction::Ok(opcode, 1, 4, "XOR B")
            }
            0xa9 => {
                self.xor_a(self.get_c());
                Instruction::Ok(opcode, 1, 4, "XOR C")
            }
            0xaa => {
                self.xor_a(self.get_d());
                Instruction::Ok(opcode, 1, 4, "XOR D")
            }
            0xab => {
                self.xor_a(self.get_e());
                Instruction::Ok(opcode, 1, 4, "XOR E")
            }
            0xac => {
                self.xor_a(self.get_h());
                Instruction::Ok(opcode, 1, 4, "XOR H")
            }
            0xad => {
                self.xor_a(self.get_l());
                Instruction::Ok(opcode, 1, 4, "XOR L")
            }
            0xae => {
                self.xor_a(mem.read_byte(self.HL));
                Instruction::Ok(opcode, 1, 8, "XOR (HL)")
            }
            0xaf => {
                self.xor_a(self.get_a());
                Instruction::Ok(opcode, 1, 4, "XOR A")
            }
            0xb0 => {
                self.or_a(self.get_b());
                Instruction::Ok(opcode, 1, 4, "OR B")
            }
            0xb1 => {
                self.or_a(self.get_c());
                Instruction::Ok(opcode, 1, 4, "OR C")
            }
            0xb2 => {
                self.or_a(self.get_d());
                Instruction::Ok(opcode, 1, 4, "OR D")
            }
            0xb3 => {
                self.or_a(self.get_e());
                Instruction::Ok(opcode, 1, 4, "OR E")
            }
            0xb4 => {
                self.or_a(self.get_h());
                Instruction::Ok(opcode, 1, 4, "OR H")
            }
            0xb5 => {
                self.or_a(self.get_l());
                Instruction::Ok(opcode, 1, 4, "OR L")
            }
            0xb6 => {
                self.or_a(mem.read_byte(self.HL));
                Instruction::Ok(opcode, 1, 8, "OR (HL)")
            }
            0xb7 => {
                self.or_a(self.get_a());
                Instruction::Ok(opcode, 1, 4, "OR A")
            }
            0xb8 => {
                self.cp_a(self.get_b());
                Instruction::Ok(opcode, 1, 4, "CP B")
            }
            0xb9 => {
                self.cp_a(self.get_c());
                Instruction::Ok(opcode, 1, 4, "CP C")
            }
            0xba => {
                self.cp_a(self.get_d());
                Instruction::Ok(opcode, 1, 4, "CP D")
            }
            0xbb => {
                self.cp_a(self.get_e());
                Instruction::Ok(opcode, 1, 4, "CP E")
            }
            0xbc => {
                self.cp_a(self.get_h());
                Instruction::Ok(opcode, 1, 4, "CP H")
            }
            0xbd => {
                self.cp_a(self.get_l());
                Instruction::Ok(opcode, 1, 4, "CP L")
            }
            0xbe => {
                self.cp_a(mem.read_byte(self.HL));
                Instruction::Ok(opcode, 1, 8, "CP (HL)")
            }
            0xbf => {
                self.cp_a(self.get_a());
                Instruction::Ok(opcode, 1, 4, "CP A")
            }
            0xc0 => {
                if !self.get_flag(Flag::Z) {
                    self.PC = self.pop_sp(mem);
                    return Instruction::Ok(opcode, 0, 20, "RET NZ");
                }
                Instruction::Ok(opcode, 1, 8, "RET NZ")
            }
            0xc1 => {
                self.BC = self.pop_sp(mem);
                Instruction::Ok(opcode, 1, 12, "POP BC")
            }
            0xc2 => {
                if !self.get_flag(Flag::Z) {
                    self.PC = self.get_nn(mem);
                    return Instruction::Ok(opcode, 0, 16, "JP NZ, a16");
                }
                Instruction::Ok(opcode, 3, 12, "JP NZ, a16")
            }
            0xc3 => {
                let nn = self.get_nn(mem);
                self.PC = nn;
                Instruction::Ok(opcode, 0, 16, "JP a16")
            }
            0xc4 => {
                if !self.get_flag(Flag::Z) {
                    self.call(mem);
                    return Instruction::Ok(opcode, 3, 24, "CALL NZ, a16");
                }
                Instruction::Ok(opcode, 3, 12, "CALL NZ, a16")
            }
            0xc5 => {
                self.push_sp(mem, self.BC);
                Instruction::Ok(opcode, 1, 4, "PUSH BC")
            }
            0xc6 => {
                self.add_a(self.get_n(mem));
                Instruction::Ok(opcode, 2, 2, "ADD, d8")
            }
            0xc7 => {
                self.rst(mem, 0x00);
                Instruction::Ok(opcode, 0, 16, "RST 0")
            }
            0xc8 => {
                if self.get_flag(Flag::Z) {
                    self.PC = self.pop_sp(mem);
                    return Instruction::Ok(opcode, 1, 20, "RET Z");
                }
                Instruction::Ok(opcode, 1, 8, "RET Z")
            }
            0xc9 => {
                self.PC = self.pop_sp(mem);
                return Instruction::Ok(opcode, 0, 4, "RET");
            }
            0xca => {
                if self.get_flag(Flag::Z) {
                    self.PC = self.get_nn(mem);
                    return Instruction::Ok(opcode, 0, 16, "JP Z, a16");
                }
                Instruction::Ok(opcode, 3, 12, "JP Z, a16")
            }
            0xcb => Instruction::Invalid(opcode), // CB
            0xcc => {
                if self.get_flag(Flag::Z) {
                    self.call(mem);
                    return Instruction::Ok(opcode, 3, 24, "CALL Z, a16");
                }
                Instruction::Ok(opcode, 3, 12, "CALL NZ, a16")
            }
            0xcd => {
                self.call(mem);
                Instruction::Ok(opcode, 3, 24, "CALL a16")
            }
            0xce => {
                self.adc_a(self.get_n(mem));
                Instruction::Ok(opcode, 2, 2, "ADC A, d8")
            }
            0xcf => {
                self.rst(mem, 0x08);
                Instruction::Ok(opcode, 0, 16, "RST 1")
            }
            0xd0 => {
                if !self.get_flag(Flag::C) {
                    self.PC = self.pop_sp(mem);
                    return Instruction::Ok(opcode, 1, 20, "RET NC");
                }
                Instruction::Ok(opcode, 1, 8, "RET NC")
            }
            0xd1 => {
                self.DE = self.pop_sp(mem);
                Instruction::Ok(opcode, 1, 12, "POP DE")
            }
            0xd2 => {
                if !self.get_flag(Flag::C) {
                    self.PC = self.get_nn(mem);
                    return Instruction::Ok(opcode, 0, 16, "JP NC, a16");
                }
                Instruction::Ok(opcode, 3, 12, "JP NC, a16")
            }
            0xd3 => Instruction::Invalid(opcode),
            0xd4 => {
                if !self.get_flag(Flag::Z) {
                    self.call(mem);
                    return Instruction::Ok(opcode, 3, 24, "CALL NZ, a16");
                }
                Instruction::Ok(opcode, 3, 12, "CALL NZ, a16")
            }
            0xd5 => {
                self.push_sp(mem, self.DE);
                Instruction::Ok(opcode, 1, 4, "PUSH DE")
            }
            0xd6 => {
                self.sub_a(self.get_n(mem));
                Instruction::Ok(opcode, 2, 2, "SUB d8")
            }
            0xd7 => {
                self.rst(mem, 0x10);
                Instruction::Ok(opcode, 0, 16, "RST 2")
            }
            0xd8 => {
                if self.get_flag(Flag::C) {
                    self.PC = self.pop_sp(mem);
                    return Instruction::Ok(opcode, 1, 20, "RET C");
                }
                Instruction::Ok(opcode, 1, 8, "RET C")
            }
            0xd9 => {
                self.PC = self.pop_sp(mem);
                self.IME = true;
                Instruction::Ok(opcode, 1, 16, "RETI")
            }
            0xda => {
                if self.get_flag(Flag::C) {
                    self.PC = self.get_nn(mem);
                    return Instruction::Ok(opcode, 0, 16, "JP C, a16");
                }
                Instruction::Ok(opcode, 3, 12, "JP C, a16")
            }
            0xdb => Instruction::Invalid(opcode),
            0xdc => {
                if self.get_flag(Flag::C) {
                    self.call(mem);
                    return Instruction::Ok(opcode, 3, 24, "CALL C, a16");
                }
                Instruction::Ok(opcode, 3, 12, "CALL C, a16")
            }
            0xdd => {
                self.call(mem);
                return Instruction::Ok(opcode, 3, 24, "CALL a16");
            }
            0xde => {
                self.sbc_a(self.get_n(mem));
                Instruction::Ok(opcode, 2, 2, "SBC A, d8")
            }
            0xdf => {
                self.rst(mem, 0x18);
                Instruction::Ok(opcode, 0, 16, "RST 3")
            }
            0xe0 => {
                let val = self.get_n(mem) as u16;
                mem.write_byte(0xFF00 + val, self.get_a());
                Instruction::Ok(opcode, 2, 12, "LDH (a8),A")
            }
            0xe1 => {
                self.HL = self.pop_sp(mem);
                Instruction::Ok(opcode, 1, 12, "POP HL")
            }
            0xe2 => {
                mem.write_byte(0xFF00 + self.get_c() as u16, self.get_a());
                Instruction::Ok(opcode, 2, 8, "LD (C), A")
            }
            0xe3 => Instruction::Invalid(opcode),
            0xe4 => Instruction::Invalid(opcode),
            0xe5 => {
                self.push_sp(mem, self.HL);
                Instruction::Ok(opcode, 1, 4, "PUSH HL")
            }
            0xe6 => {
                self.and_a(self.get_n(mem));
                Instruction::Ok(opcode, 2, 2, "AND d8")
            }
            0xe7 => {
                self.rst(mem, 0x20);
                Instruction::Ok(opcode, 0, 16, "RST 5")
            }
            0xe8 => {
                self.SP = self.add_16(self.SP, self.get_n(mem) as u16);
                self.set_flag(Flag::Z, false);
                Instruction::Ok(opcode, 2, 16, "ADD SP,r8")
            }
            0xe9 => {
                self.PC = mem.read_word(self.HL);
                Instruction::Ok(opcode, 0, 16, "JP (HL)")
            }
            0xea => {
                let addr = self.get_nn(mem);
                mem.write_byte(addr, self.get_a());
                Instruction::Ok(opcode, 3, 16, "LD (a16),A")
            }
            0xeb => Instruction::Invalid(opcode),
            0xec => Instruction::Invalid(opcode),
            0xed => Instruction::Invalid(opcode),
            0xee => {
                self.xor_a(self.get_n(mem));
                Instruction::Ok(opcode, 2, 2, "XOR d8")
            }
            0xef => {
                self.rst(mem, 0x28);
                Instruction::Ok(opcode, 0, 16, "RST 5")
            }
            0xf0 => {
                let val = self.get_n(mem) as u16;
                self.set_a(mem.read_byte(0xFF00 + val));
                Instruction::Ok(opcode, 2, 12, "LDH A,(a8)")
            }
            0xf1 => {
                self.AF = self.pop_sp(mem);
                Instruction::Ok(opcode, 1, 12, "POP AF")
            }
            0xf2 => {
                self.set_a(mem.read_byte(0xFF00 + self.get_c() as u16));
                Instruction::Ok(opcode, 2, 8, "LD A, (C)")
            }
            0xf3 => {
                self.DI = true;
                Instruction::Ok(opcode, 1, 4, "DI")
            }
            0xf4 => Instruction::Invalid(opcode),
            0xf5 => {
                self.push_sp(mem, self.AF);
                Instruction::Ok(opcode, 1, 16, "PUSH AF")
            }
            0xf6 => {
                self.or_a(self.get_n(mem));
                Instruction::Ok(opcode, 2, 8, "OR d8")
            }
            0xf7 => {
                self.rst(mem, 0x30);
                Instruction::Ok(opcode, 0, 16, "RST 6")
            }
            0xf8 => {
                let r8 = self.get_n(mem) as u16;
                self.HL = self.add_16(self.SP, r8);
                self.set_flag(Flag::Z, false);
                Instruction::Ok(opcode, 2, 12, "LD HL,SP+r8")
            }
            0xf9 => {
                self.SP = self.HL;
                Instruction::Ok(opcode, 1, 8, "LD SP,HL")
            }
            0xfa => {
                let a16 = self.get_nn(mem);
                self.set_a(mem.read_byte(a16));
                Instruction::Ok(opcode, 3, 16, "LD A,(a16)")
            }
            0xfb => {
                self.EI = true;
                Instruction::Ok(opcode, 1, 4, "EI")
            }
            0xfc => Instruction::Invalid(opcode),
            0xfd => Instruction::Invalid(opcode),
            0xfe => {
                self.cp_a(self.get_n(mem));
                Instruction::Ok(opcode, 2, 8, "CP d8")
            }
            0xff => {
                self.rst(mem, 0x38);
                Instruction::Ok(opcode, 0, 16, "RST 7")
            }
        }
    }
}
