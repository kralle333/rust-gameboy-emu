use crate::memory::{self, MemoryType};

use super::{helpers::Instruction, Cpu};

impl Cpu {
    pub fn execute_cb(&mut self, opcode: u8, mem: &mut memory::Memory) -> Instruction {
        match opcode {
            0x0 => {
                let result = self.rlc(self.get_b());
                self.set_b(result);
                Instruction::Ok(opcode,2, 8, "RLC B")
            }
            0x1 => {
                let result = self.rlc(self.get_c());
                self.set_c(result);
                Instruction::Ok(opcode,2, 8, "RLC C")
            }
            0x2 => {
                let result = self.rlc(self.get_d());
                self.set_d(result);
                Instruction::Ok(opcode,2, 8, "RLC D")
            }
            0x3 => {
                let result = self.rlc(self.get_e());
                self.set_e(result);
                Instruction::Ok(opcode,2, 8, "RLC E")
            }
            0x4 => {
                let result = self.rlc(self.get_h());
                self.set_h(result);
                Instruction::Ok(opcode,2, 8, "RLC H")
            }
            0x5 => {
                let result = self.rlc(self.get_l());
                self.set_l(result);
                Instruction::Ok(opcode,2, 8, "RLC L")
            }
            0x6 => {
                mem.write_byte(self.HL, self.rlc(mem.read_byte(self.HL)));
                Instruction::Ok(opcode,2, 16, "RLC (HL)")
            }
            0x7 => {
                let result = self.rlc(self.get_a());
                self.set_a(result);
                Instruction::Ok(opcode,2, 8, "RLC A")
            }
            0x8 => {
                let result = self.rrc(self.get_b());
                self.set_b(result);
                Instruction::Ok(opcode,2, 8, "RRC B")
            }
            0x9 => {
                let result = self.rrc(self.get_c());
                self.set_c(result);
                Instruction::Ok(opcode,2, 8, "RRC C")
            }
            0xa => {
                let result = self.rrc(self.get_d());
                self.set_d(result);
                Instruction::Ok(opcode,2, 8, "RRC D")
            }
            0xb => {
                let result = self.rrc(self.get_e());
                self.set_e(result);
                Instruction::Ok(opcode,2, 8, "RRC E")
            }
            0xc => {
                let result = self.rrc(self.get_h());
                self.set_h(result);
                Instruction::Ok(opcode,2, 8, "RRC H")
            }
            0xd => {
                let result = self.rrc(self.get_l());
                self.set_l(result);
                Instruction::Ok(opcode,2, 8, "RRC L")
            }
            0xe => {
                mem.write_byte(self.HL, self.rrc(mem.read_byte(self.HL)));
                Instruction::Ok(opcode,2, 16, "RRC (HL)")
            }
            0xf => {
                let result = self.rrc(self.get_a());
                self.set_a(result);
                Instruction::Ok(opcode,2, 8, "RRC A")
            }
            0x10 => {
                let result = self.rl(self.get_b());
                self.set_b(result);
                Instruction::Ok(opcode,2, 8, "RL B")
            }
            0x11 => {
                let result = self.rl(self.get_c());
                self.set_c(result);
                Instruction::Ok(opcode,2, 8, "RL C")
            }
            0x12 => {
                let result = self.rl(self.get_d());
                self.set_d(result);
                Instruction::Ok(opcode,2, 8, "RL D")
            }
            0x13 => {
                let result = self.rl(self.get_e());
                self.set_e(result);
                Instruction::Ok(opcode,2, 8, "RL E")
            }
            0x14 => {
                let result = self.rl(self.get_h());
                self.set_h(result);
                Instruction::Ok(opcode,2, 8, "RL H")
            }
            0x15 => {
                let result = self.rl(self.get_l());
                self.set_l(result);
                Instruction::Ok(opcode,2, 8, "RL L")
            }
            0x16 => {
                mem.write_byte(self.HL, self.rl(mem.read_byte(self.HL)));
                Instruction::Ok(opcode,2, 16, "RL (HL)")
            }
            0x17 => {
                let result = self.rl(self.get_a());
                self.set_a(result);
                Instruction::Ok(opcode,2, 8, "RL A")
            }
            0x18 => {
                let result = self.rr(self.get_b());
                self.set_b(result);
                Instruction::Ok(opcode,2, 8, "RR B")
            }
            0x19 => {
                let result = self.rr(self.get_c());
                self.set_c(result);
                Instruction::Ok(opcode,2, 8, "RR C")
            }
            0x1a => {
                let result = self.rr(self.get_d());
                self.set_d(result);
                Instruction::Ok(opcode,2, 8, "RR D")
            }
            0x1b => {
                let result = self.rr(self.get_e());
                self.set_e(result);
                Instruction::Ok(opcode,2, 8, "RR E")
            }
            0x1c => {
                let result = self.rr(self.get_h());
                self.set_h(result);
                Instruction::Ok(opcode,2, 8, "RR H")
            }
            0x1d => {
                let result = self.rr(self.get_l());
                self.set_l(result);
                Instruction::Ok(opcode,2, 8, "RR L")
            }
            0x1e => {
                mem.write_byte(self.HL, self.rr(mem.read_byte(self.HL)));
                Instruction::Ok(opcode,2, 16, "RR (HL)")
            }
            0x1f => {
                let result = self.rr(self.get_a());
                self.set_a(result);
                Instruction::Ok(opcode,2, 8, "RR A")
            }
            0x20 => {
                let result = self.sla(self.get_b());
                self.set_b(result);
                Instruction::Ok(opcode,2, 8, "SLA B")
            }
            0x21 => {
                let result = self.sla(self.get_c());
                self.set_c(result);
                Instruction::Ok(opcode,2, 8, "SLA C")
            }
            0x22 => {
                let result = self.sla(self.get_d());
                self.set_d(result);
                Instruction::Ok(opcode,2, 8, "SLA D")
            }
            0x23 => {
                let result = self.sla(self.get_e());
                self.set_e(result);
                Instruction::Ok(opcode,2, 8, "SLA E")
            }
            0x24 => {
                let result = self.sla(self.get_h());
                self.set_h(result);
                Instruction::Ok(opcode,2, 8, "SLA H")
            }
            0x25 => {
                let result = self.sla(self.get_l());
                self.set_l(result);
                Instruction::Ok(opcode,2, 8, "SLA L")
            }
            0x26 => {
                mem.write_byte(self.HL, self.sla(mem.read_byte(self.HL)));
                Instruction::Ok(opcode,2, 16, "SLA (HL)")
            }
            0x27 => {
                let result = self.sla(self.get_a());
                self.set_a(result);
                Instruction::Ok(opcode,2, 8, "SLA A")
            }
            0x28 => {
                let result = self.sra(self.get_b());
                self.set_b(result);
                Instruction::Ok(opcode,2, 8, "SRA B")
            }
            0x29 => {
                let result = self.sra(self.get_c());
                self.set_c(result);
                Instruction::Ok(opcode,2, 8, "SRA C")
            }
            0x2a => {
                let result = self.sra(self.get_d());
                self.set_d(result);
                Instruction::Ok(opcode,2, 8, "SRA D")
            }
            0x2b => {
                let result = self.sra(self.get_e());
                self.set_e(result);
                Instruction::Ok(opcode,2, 8, "SRA E")
            }
            0x2c => {
                let result = self.sra(self.get_h());
                self.set_h(result);
                Instruction::Ok(opcode,2, 8, "SRA H")
            }
            0x2d => {
                let result = self.sra(self.get_l());
                self.set_l(result);
                Instruction::Ok(opcode,2, 8, "SRA L")
            }
            0x2e => {
                mem.write_byte(self.HL, self.sra(mem.read_byte(self.HL)));
                Instruction::Ok(opcode,2, 16, "SRA (HL)")
            }
            0x2f => {
                let result = self.sra(self.get_a());
                self.set_a(result);
                Instruction::Ok(opcode,2, 8, "SRA A")
            }
            0x30 => {
                let result = self.swap(self.get_b());
                self.set_b(result);
                Instruction::Ok(opcode,2, 8, "SWAP B")
            }
            0x31 => {
                let result = self.swap(self.get_c());
                self.set_c(result);
                Instruction::Ok(opcode,2, 8, "SWAP C")
            }
            0x32 => {
                let result = self.swap(self.get_d());
                self.set_d(result);
                Instruction::Ok(opcode,2, 8, "SWAP D")
            }
            0x33 => {
                let result = self.swap(self.get_e());
                self.set_e(result);
                Instruction::Ok(opcode,2, 8, "SWAP E")
            }
            0x34 => {
                let result = self.swap(self.get_h());
                self.set_h(result);
                Instruction::Ok(opcode,2, 8, "SWAP H")
            }
            0x35 => {
                let result = self.swap(self.get_l());
                self.set_l(result);
                Instruction::Ok(opcode,2, 8, "SWAP L")
            }
            0x36 => {
                mem.write_byte(self.HL, self.swap(mem.read_byte(self.HL)));
                Instruction::Ok(opcode,2, 16, "SWAP (HL)")
            }
            0x37 => {
                let result = self.swap(self.get_a());
                self.set_a(result);
                Instruction::Ok(opcode,2, 8, "SWAP A")
            }
            0x38 => {
                let result = self.srl(self.get_b());
                self.set_b(result);
                Instruction::Ok(opcode,2, 8, "SRL B")
            }
            0x39 => {
                let result = self.srl(self.get_c());
                self.set_c(result);
                Instruction::Ok(opcode,2, 8, "SRL C")
            }
            0x3a => {
                let result = self.srl(self.get_d());
                self.set_d(result);
                Instruction::Ok(opcode,2, 8, "SRL D")
            }
            0x3b => {
                let result = self.srl(self.get_e());
                self.set_e(result);
                Instruction::Ok(opcode,2, 8, "SRL E")
            }
            0x3c => {
                let result = self.srl(self.get_h());
                self.set_h(result);
                Instruction::Ok(opcode,2, 8, "SRL H")
            }
            0x3d => {
                let result = self.srl(self.get_l());
                self.set_l(result);
                Instruction::Ok(opcode,2, 8, "SRL L")
            }
            0x3e => {
                mem.write_byte(self.HL, self.srl(mem.read_byte(self.HL)));
                Instruction::Ok(opcode,2, 16, "SRL (HL)")
            }
            0x3f => {
                let result = self.srl(self.get_a());
                self.set_a(result);
                Instruction::Ok(opcode,2, 8, "SRL A")
            }

            0x40 => {
                self.bit(0, self.get_b());
                Instruction::Ok(opcode,2, 8, "BIT 0, B")
            }
            0x41 => {
                self.bit(0, self.get_c());
                Instruction::Ok(opcode,2, 8, "BIT 0, C")
            }
            0x42 => {
                self.bit(0, self.get_d());
                Instruction::Ok(opcode,2, 8, "BIT 0, D")
            }
            0x43 => {
                self.bit(0, self.get_e());
                Instruction::Ok(opcode,2, 8, "BIT 0, E")
            }
            0x44 => {
                self.bit(0, self.get_h());
                Instruction::Ok(opcode,2, 8, "BIT 0, H")
            }
            0x45 => {
                self.bit(0, self.get_l());
                Instruction::Ok(opcode,2, 8, "BIT 0, L")
            }
            0x46 => {
                self.bit(0, mem.read_byte(self.HL));
                Instruction::Ok(opcode,2, 16, "BIT 0, (HL)")
            }
            0x47 => {
                self.bit(0, self.get_a());
                Instruction::Ok(opcode,2, 8, "BIT 0, A")
            }
            0x48 => {
                self.bit(1, self.get_b());
                Instruction::Ok(opcode,2, 8, "BIT 1, B")
            }
            0x49 => {
                self.bit(1, self.get_c());
                Instruction::Ok(opcode,2, 8, "BIT 1, C")
            }
            0x4a => {
                self.bit(1, self.get_d());
                Instruction::Ok(opcode,2, 8, "BIT 1, D")
            }
            0x4b => {
                self.bit(1, self.get_e());
                Instruction::Ok(opcode,2, 8, "BIT 1, E")
            }
            0x4c => {
                self.bit(1, self.get_h());
                Instruction::Ok(opcode,2, 8, "BIT 1, H")
            }
            0x4d => {
                self.bit(1, self.get_l());
                Instruction::Ok(opcode,2, 8, "BIT 1, L")
            }
            0x4e => {
                self.bit(1, mem.read_byte(self.HL));
                Instruction::Ok(opcode,2, 16, "BIT 1, (HL)")
            }
            0x4f => {
                self.bit(1, self.get_a());
                Instruction::Ok(opcode,2, 8, "BIT 1, A")
            }
            0x50 => {
                self.bit(2, self.get_b());
                Instruction::Ok(opcode,2, 8, "BIT 2, B")
            }
            0x51 => {
                self.bit(2, self.get_c());
                Instruction::Ok(opcode,2, 8, "BIT 2, C")
            }
            0x52 => {
                self.bit(2, self.get_d());
                Instruction::Ok(opcode,2, 8, "BIT 2, D")
            }
            0x53 => {
                self.bit(2, self.get_e());
                Instruction::Ok(opcode,2, 8, "BIT 2, E")
            }
            0x54 => {
                self.bit(2, self.get_h());
                Instruction::Ok(opcode,2, 8, "BIT 2, H")
            }
            0x55 => {
                self.bit(2, self.get_l());
                Instruction::Ok(opcode,2, 8, "BIT 2, L")
            }
            0x56 => {
                self.bit(2, mem.read_byte(self.HL));
                Instruction::Ok(opcode,2, 16, "BIT 2, (HL)")
            }
            0x57 => {
                self.bit(2, self.get_a());
                Instruction::Ok(opcode,2, 8, "BIT 2, A")
            }
            0x58 => {
                self.bit(3, self.get_b());
                Instruction::Ok(opcode,2, 8, "BIT 3, B")
            }
            0x59 => {
                self.bit(3, self.get_c());
                Instruction::Ok(opcode,2, 8, "BIT 3, C")
            }
            0x5a => {
                self.bit(3, self.get_d());
                Instruction::Ok(opcode,2, 8, "BIT 3, D")
            }
            0x5b => {
                self.bit(3, self.get_e());
                Instruction::Ok(opcode,2, 8, "BIT 3, E")
            }
            0x5c => {
                self.bit(3, self.get_h());
                Instruction::Ok(opcode,2, 8, "BIT 3, H")
            }
            0x5d => {
                self.bit(3, self.get_l());
                Instruction::Ok(opcode,2, 8, "BIT 3, L")
            }
            0x5e => {
                self.bit(3, mem.read_byte(self.HL));
                Instruction::Ok(opcode,2, 16, "BIT 3, (HL)")
            }
            0x5f => {
                self.bit(3, self.get_a());
                Instruction::Ok(opcode,2, 8, "BIT 3, A")
            }
            0x60 => {
                self.bit(4, self.get_b());
                Instruction::Ok(opcode,2, 8, "BIT 4, B")
            }
            0x61 => {
                self.bit(4, self.get_c());
                Instruction::Ok(opcode,2, 8, "BIT 4, C")
            }
            0x62 => {
                self.bit(4, self.get_d());
                Instruction::Ok(opcode,2, 8, "BIT 4, D")
            }
            0x63 => {
                self.bit(4, self.get_e());
                Instruction::Ok(opcode,2, 8, "BIT 4, E")
            }
            0x64 => {
                self.bit(4, self.get_h());
                Instruction::Ok(opcode,2, 8, "BIT 4, H")
            }
            0x65 => {
                self.bit(4, self.get_l());
                Instruction::Ok(opcode,2, 8, "BIT 4, L")
            }
            0x66 => {
                self.bit(4, mem.read_byte(self.HL));
                Instruction::Ok(opcode,2, 16, "BIT 4, (HL)")
            }
            0x67 => {
                self.bit(4, self.get_a());
                Instruction::Ok(opcode,2, 8, "BIT 4, A")
            }
            0x68 => {
                self.bit(5, self.get_b());
                Instruction::Ok(opcode,2, 8, "BIT 5, B")
            }
            0x69 => {
                self.bit(5, self.get_c());
                Instruction::Ok(opcode,2, 8, "BIT 5, C")
            }
            0x6a => {
                self.bit(5, self.get_d());
                Instruction::Ok(opcode,2, 8, "BIT 5, D")
            }
            0x6b => {
                self.bit(5, self.get_e());
                Instruction::Ok(opcode,2, 8, "BIT 5, E")
            }
            0x6c => {
                self.bit(5, self.get_h());
                Instruction::Ok(opcode,2, 8, "BIT 5, H")
            }
            0x6d => {
                self.bit(5, self.get_l());
                Instruction::Ok(opcode,2, 8, "BIT 5, L")
            }
            0x6e => {
                self.bit(5, mem.read_byte(self.HL));
                Instruction::Ok(opcode,2, 16, "BIT 5, (HL)")
            }
            0x6f => {
                self.bit(5, self.get_a());
                Instruction::Ok(opcode,2, 8, "BIT 5, A")
            }
            0x70 => {
                self.bit(6, self.get_b());
                Instruction::Ok(opcode,2, 8, "BIT 6, B")
            }
            0x71 => {
                self.bit(6, self.get_c());
                Instruction::Ok(opcode,2, 8, "BIT 6, C")
            }
            0x72 => {
                self.bit(6, self.get_d());
                Instruction::Ok(opcode,2, 8, "BIT 6, D")
            }
            0x73 => {
                self.bit(6, self.get_e());
                Instruction::Ok(opcode,2, 8, "BIT 6, E")
            }
            0x74 => {
                self.bit(6, self.get_h());
                Instruction::Ok(opcode,2, 8, "BIT 6, H")
            }
            0x75 => {
                self.bit(6, self.get_l());
                Instruction::Ok(opcode,2, 8, "BIT 6, L")
            }
            0x76 => {
                self.bit(6, mem.read_byte(self.HL));
                Instruction::Ok(opcode,2, 16, "BIT 6, (HL)")
            }
            0x77 => {
                self.bit(6, self.get_a());
                Instruction::Ok(opcode,2, 8, "BIT 6, A")
            }
            0x78 => {
                self.bit(7, self.get_b());
                Instruction::Ok(opcode,2, 8, "BIT 7, B")
            }
            0x79 => {
                self.bit(7, self.get_c());
                Instruction::Ok(opcode,2, 8, "BIT 7, C")
            }
            0x7a => {
                self.bit(7, self.get_d());
                Instruction::Ok(opcode,2, 8, "BIT 7, D")
            }
            0x7b => {
                self.bit(7, self.get_e());
                Instruction::Ok(opcode,2, 8, "BIT 7, E")
            }
            0x7c => {
                self.bit(7, self.get_h());
                Instruction::Ok(opcode,2, 8, "BIT 7, H")
            }
            0x7d => {
                self.bit(7, self.get_l());
                Instruction::Ok(opcode,2, 8, "BIT 7, L")
            }
            0x7e => {
                self.bit(7, mem.read_byte(self.HL));
                Instruction::Ok(opcode,2, 16, "BIT 7, (HL)")
            }
            0x7f => {
                self.bit(7, self.get_a());
                Instruction::Ok(opcode,2, 8, "BIT 7, A")
            }
            0x80 => {
                let result = self.res_bit(0, self.get_b());
                self.set_b(result);
                Instruction::Ok(opcode,2, 8, "RES 0, B")
            }
            0x81 => {
                let result = self.res_bit(0, self.get_c());
                self.set_c(result);
                Instruction::Ok(opcode,2, 8, "RES 0, C")
            }
            0x82 => {
                let result = self.res_bit(0, self.get_d());
                self.set_d(result);
                Instruction::Ok(opcode,2, 8, "RES 0, D")
            }
            0x83 => {
                let result = self.res_bit(0, self.get_e());
                self.set_e(result);
                Instruction::Ok(opcode,2, 8, "RES 0, E")
            }
            0x84 => {
                let result = self.res_bit(0, self.get_h());
                self.set_h(result);
                Instruction::Ok(opcode,2, 8, "RES 0, H")
            }
            0x85 => {
                let result = self.res_bit(0, self.get_l());
                self.set_l(result);
                Instruction::Ok(opcode,2, 8, "RES 0, L")
            }
            0x86 => {
                mem.write_byte(self.HL, self.res_bit(0, mem.read_byte(self.HL)));
                Instruction::Ok(opcode,2, 16, "RES 0, (HL)")
            }
            0x87 => {
                let result = self.res_bit(0, self.get_a());
                self.set_a(result);
                Instruction::Ok(opcode,2, 8, "RES 0, A")
            }
            0x88 => {
                let result = self.res_bit(1, self.get_b());
                self.set_b(result);
                Instruction::Ok(opcode,2, 8, "RES 1, B")
            }
            0x89 => {
                let result = self.res_bit(1, self.get_c());
                self.set_c(result);
                Instruction::Ok(opcode,2, 8, "RES 1, C")
            }
            0x8a => {
                let result = self.res_bit(1, self.get_d());
                self.set_d(result);
                Instruction::Ok(opcode,2, 8, "RES 1, D")
            }
            0x8b => {
                let result = self.res_bit(1, self.get_e());
                self.set_e(result);
                Instruction::Ok(opcode,2, 8, "RES 1, E")
            }
            0x8c => {
                let result = self.res_bit(1, self.get_h());
                self.set_h(result);
                Instruction::Ok(opcode,2, 8, "RES 1, H")
            }
            0x8d => {
                let result = self.res_bit(1, self.get_l());
                self.set_l(result);
                Instruction::Ok(opcode,2, 8, "RES 1, L")
            }
            0x8e => {
                mem.write_byte(self.HL, self.res_bit(1, mem.read_byte(self.HL)));
                Instruction::Ok(opcode,2, 16, "RES 1, (HL)")
            }
            0x8f => {
                let result = self.res_bit(1, self.get_a());
                self.set_a(result);
                Instruction::Ok(opcode,2, 8, "RES 1, A")
            }
            0x90 => {
                let result = self.res_bit(2, self.get_b());
                self.set_b(result);
                Instruction::Ok(opcode,2, 8, "RES 2, B")
            }
            0x91 => {
                let result = self.res_bit(2, self.get_c());
                self.set_c(result);
                Instruction::Ok(opcode,2, 8, "RES 2, C")
            }
            0x92 => {
                let result = self.res_bit(2, self.get_d());
                self.set_d(result);
                Instruction::Ok(opcode,2, 8, "RES 2, D")
            }
            0x93 => {
                let result = self.res_bit(2, self.get_e());
                self.set_e(result);
                Instruction::Ok(opcode,2, 8, "RES 2, E")
            }
            0x94 => {
                let result = self.res_bit(2, self.get_h());
                self.set_h(result);
                Instruction::Ok(opcode,2, 8, "RES 2, H")
            }
            0x95 => {
                let result = self.res_bit(2, self.get_l());
                self.set_l(result);
                Instruction::Ok(opcode,2, 8, "RES 2, L")
            }
            0x96 => {
                mem.write_byte(self.HL, self.res_bit(2, mem.read_byte(self.HL)));
                Instruction::Ok(opcode,2, 16, "RES 2, (HL)")
            }
            0x97 => {
                let result = self.res_bit(2, self.get_a());
                self.set_a(result);
                Instruction::Ok(opcode,2, 8, "RES 2, A")
            }
            0x98 => {
                let result = self.res_bit(3, self.get_b());
                self.set_b(result);
                Instruction::Ok(opcode,2, 8, "RES 3, B")
            }
            0x99 => {
                let result = self.res_bit(3, self.get_c());
                self.set_c(result);
                Instruction::Ok(opcode,2, 8, "RES 3, C")
            }
            0x9a => {
                let result = self.res_bit(3, self.get_d());
                self.set_d(result);
                Instruction::Ok(opcode,2, 8, "RES 3, D")
            }
            0x9b => {
                let result = self.res_bit(3, self.get_e());
                self.set_e(result);
                Instruction::Ok(opcode,2, 8, "RES 3, E")
            }
            0x9c => {
                let result = self.res_bit(3, self.get_h());
                self.set_h(result);
                Instruction::Ok(opcode,2, 8, "RES 3, H")
            }
            0x9d => {
                let result = self.res_bit(3, self.get_l());
                self.set_l(result);
                Instruction::Ok(opcode,2, 8, "RES 3, L")
            }
            0x9e => {
                mem.write_byte(self.HL, self.res_bit(3, mem.read_byte(self.HL)));
                Instruction::Ok(opcode,2, 16, "RES 3, (HL)")
            }
            0x9f => {
                let result = self.res_bit(3, self.get_a());
                self.set_a(result);
                Instruction::Ok(opcode,2, 8, "RES 3, A")
            }
            0xa0 => {
                let result = self.res_bit(4, self.get_b());
                self.set_b(result);
                Instruction::Ok(opcode,2, 8, "RES 4, B")
            }
            0xa1 => {
                let result = self.res_bit(4, self.get_c());
                self.set_c(result);
                Instruction::Ok(opcode,2, 8, "RES 4, C")
            }
            0xa2 => {
                let result = self.res_bit(4, self.get_d());
                self.set_d(result);
                Instruction::Ok(opcode,2, 8, "RES 4, D")
            }
            0xa3 => {
                let result = self.res_bit(4, self.get_e());
                self.set_e(result);
                Instruction::Ok(opcode,2, 8, "RES 4, E")
            }
            0xa4 => {
                let result = self.res_bit(4, self.get_h());
                self.set_h(result);
                Instruction::Ok(opcode,2, 8, "RES 4, H")
            }
            0xa5 => {
                let result = self.res_bit(4, self.get_l());
                self.set_l(result);
                Instruction::Ok(opcode,2, 8, "RES 4, L")
            }
            0xa6 => {
                mem.write_byte(self.HL, self.res_bit(4, mem.read_byte(self.HL)));
                Instruction::Ok(opcode,2, 16, "RES 4, (HL)")
            }
            0xa7 => {
                let result = self.res_bit(4, self.get_a());
                self.set_a(result);
                Instruction::Ok(opcode,2, 8, "RES 4, A")
            }
            0xa8 => {
                let result = self.res_bit(5, self.get_b());
                self.set_b(result);
                Instruction::Ok(opcode,2, 8, "RES 5, B")
            }
            0xa9 => {
                let result = self.res_bit(5, self.get_c());
                self.set_c(result);
                Instruction::Ok(opcode,2, 8, "RES 5, C")
            }
            0xaa => {
                let result = self.res_bit(5, self.get_d());
                self.set_d(result);
                Instruction::Ok(opcode,2, 8, "RES 5, D")
            }
            0xab => {
                let result = self.res_bit(5, self.get_e());
                self.set_e(result);
                Instruction::Ok(opcode,2, 8, "RES 5, E")
            }
            0xac => {
                let result = self.res_bit(5, self.get_h());
                self.set_h(result);
                Instruction::Ok(opcode,2, 8, "RES 5, H")
            }
            0xad => {
                let result = self.res_bit(5, self.get_l());
                self.set_l(result);
                Instruction::Ok(opcode,2, 8, "RES 5, L")
            }
            0xae => {
                mem.write_byte(self.HL, self.res_bit(5, mem.read_byte(self.HL)));
                Instruction::Ok(opcode,2, 16, "RES 5, (HL)")
            }
            0xaf => {
                let result = self.res_bit(5, self.get_a());
                self.set_a(result);
                Instruction::Ok(opcode,2, 8, "RES 5, A")
            }
            0xb0 => {
                let result = self.res_bit(6, self.get_b());
                self.set_b(result);
                Instruction::Ok(opcode,2, 8, "RES 6, B")
            }
            0xb1 => {
                let result = self.res_bit(6, self.get_c());
                self.set_c(result);
                Instruction::Ok(opcode,2, 8, "RES 6, C")
            }
            0xb2 => {
                let result = self.res_bit(6, self.get_d());
                self.set_d(result);
                Instruction::Ok(opcode,2, 8, "RES 6, D")
            }
            0xb3 => {
                let result = self.res_bit(6, self.get_e());
                self.set_e(result);
                Instruction::Ok(opcode,2, 8, "RES 6, E")
            }
            0xb4 => {
                let result = self.res_bit(6, self.get_h());
                self.set_h(result);
                Instruction::Ok(opcode,2, 8, "RES 6, H")
            }
            0xb5 => {
                let result = self.res_bit(6, self.get_l());
                self.set_l(result);
                Instruction::Ok(opcode,2, 8, "RES 6, L")
            }
            0xb6 => {
                mem.write_byte(self.HL, self.res_bit(6, mem.read_byte(self.HL)));
                Instruction::Ok(opcode,2, 16, "RES 6, (HL)")
            }
            0xb7 => {
                let result = self.res_bit(6, self.get_a());
                self.set_a(result);
                Instruction::Ok(opcode,2, 8, "RES 6, A")
            }
            0xb8 => {
                let result = self.res_bit(7, self.get_b());
                self.set_b(result);
                Instruction::Ok(opcode,2, 8, "RES 7, B")
            }
            0xb9 => {
                let result = self.res_bit(7, self.get_c());
                self.set_c(result);
                Instruction::Ok(opcode,2, 8, "RES 7, C")
            }
            0xba => {
                let result = self.res_bit(7, self.get_d());
                self.set_d(result);
                Instruction::Ok(opcode,2, 8, "RES 7, D")
            }
            0xbb => {
                let result = self.res_bit(7, self.get_e());
                self.set_e(result);
                Instruction::Ok(opcode,2, 8, "RES 7, E")
            }
            0xbc => {
                let result = self.res_bit(7, self.get_h());
                self.set_h(result);
                Instruction::Ok(opcode,2, 8, "RES 7, H")
            }
            0xbd => {
                let result = self.res_bit(7, self.get_l());
                self.set_l(result);
                Instruction::Ok(opcode,2, 8, "RES 7, L")
            }
            0xbe => {
                mem.write_byte(self.HL, self.res_bit(7, mem.read_byte(self.HL)));
                Instruction::Ok(opcode,2, 16, "RES 7, (HL)")
            }
            0xbf => {
                let result = self.res_bit(7, self.get_a());
                self.set_a(result);
                Instruction::Ok(opcode,2, 8, "RES 7, A")
            }
            0xc0 => {
                let result = self.set_bit(0, self.get_b());
                self.set_b(result);
                Instruction::Ok(opcode,2, 8, "SET 0, B")
            }
            0xc1 => {
                let result = self.set_bit(0, self.get_c());
                self.set_c(result);
                Instruction::Ok(opcode,2, 8, "SET 0, C")
            }
            0xc2 => {
                let result = self.set_bit(0, self.get_d());
                self.set_d(result);
                Instruction::Ok(opcode,2, 8, "SET 0, D")
            }
            0xc3 => {
                let result = self.set_bit(0, self.get_e());
                self.set_e(result);
                Instruction::Ok(opcode,2, 8, "SET 0, E")
            }
            0xc4 => {
                let result = self.set_bit(0, self.get_h());
                self.set_h(result);
                Instruction::Ok(opcode,2, 8, "SET 0, H")
            }
            0xc5 => {
                let result = self.set_bit(0, self.get_l());
                self.set_l(result);
                Instruction::Ok(opcode,2, 8, "SET 0, L")
            }
            0xc6 => {
                mem.write_byte(self.HL, self.set_bit(0, mem.read_byte(self.HL)));
                Instruction::Ok(opcode,2, 16, "SET 0, (HL)")
            }
            0xc7 => {
                let result = self.set_bit(0, self.get_a());
                self.set_a(result);
                Instruction::Ok(opcode,2, 8, "SET 0, A")
            }
            0xc8 => {
                let result = self.set_bit(1, self.get_b());
                self.set_b(result);
                Instruction::Ok(opcode,2, 8, "SET 1, B")
            }
            0xc9 => {
                let result = self.set_bit(1, self.get_c());
                self.set_c(result);
                Instruction::Ok(opcode,2, 8, "SET 1, C")
            }
            0xca => {
                let result = self.set_bit(1, self.get_d());
                self.set_d(result);
                Instruction::Ok(opcode,2, 8, "SET 1, D")
            }
            0xcb => {
                let result = self.set_bit(1, self.get_e());
                self.set_e(result);
                Instruction::Ok(opcode,2, 8, "SET 1, E")
            }
            0xcc => {
                let result = self.set_bit(1, self.get_h());
                self.set_h(result);
                Instruction::Ok(opcode,2, 8, "SET 1, H")
            }
            0xcd => {
                let result = self.set_bit(1, self.get_l());
                self.set_l(result);
                Instruction::Ok(opcode,2, 8, "SET 1, L")
            }
            0xce => {
                mem.write_byte(self.HL, self.set_bit(1, mem.read_byte(self.HL)));
                Instruction::Ok(opcode,2, 16, "SET 1, (HL)")
            }
            0xcf => {
                let result = self.set_bit(1, self.get_a());
                self.set_a(result);
                Instruction::Ok(opcode,2, 8, "SET 1, A")
            }
            0xd0 => {
                let result = self.set_bit(2, self.get_b());
                self.set_b(result);
                Instruction::Ok(opcode,2, 8, "SET 2, B")
            }
            0xd1 => {
                let result = self.set_bit(2, self.get_c());
                self.set_c(result);
                Instruction::Ok(opcode,2, 8, "SET 2, C")
            }
            0xd2 => {
                let result = self.set_bit(2, self.get_d());
                self.set_d(result);
                Instruction::Ok(opcode,2, 8, "SET 2, D")
            }
            0xd3 => {
                let result = self.set_bit(2, self.get_e());
                self.set_e(result);
                Instruction::Ok(opcode,2, 8, "SET 2, E")
            }
            0xd4 => {
                let result = self.set_bit(2, self.get_h());
                self.set_h(result);
                Instruction::Ok(opcode,2, 8, "SET 2, H")
            }
            0xd5 => {
                let result = self.set_bit(2, self.get_l());
                self.set_l(result);
                Instruction::Ok(opcode,2, 8, "SET 2, L")
            }
            0xd6 => {
                mem.write_byte(self.HL, self.set_bit(2, mem.read_byte(self.HL)));
                Instruction::Ok(opcode,2, 16, "SET 2, (HL)")
            }
            0xd7 => {
                let result = self.set_bit(2, self.get_a());
                self.set_a(result);
                Instruction::Ok(opcode,2, 8, "SET 2, A")
            }
            0xd8 => {
                let result = self.set_bit(3, self.get_b());
                self.set_b(result);
                Instruction::Ok(opcode,2, 8, "SET 3, B")
            }
            0xd9 => {
                let result = self.set_bit(3, self.get_c());
                self.set_c(result);
                Instruction::Ok(opcode,2, 8, "SET 3, C")
            }
            0xda => {
                let result = self.set_bit(3, self.get_d());
                self.set_d(result);
                Instruction::Ok(opcode,2, 8, "SET 3, D")
            }
            0xdb => {
                let result = self.set_bit(3, self.get_e());
                self.set_e(result);
                Instruction::Ok(opcode,2, 8, "SET 3, E")
            }
            0xdc => {
                let result = self.set_bit(3, self.get_h());
                self.set_h(result);
                Instruction::Ok(opcode,2, 8, "SET 3, H")
            }
            0xdd => {
                let result = self.set_bit(3, self.get_l());
                self.set_l(result);
                Instruction::Ok(opcode,2, 8, "SET 3, L")
            }
            0xde => {
                mem.write_byte(self.HL, self.set_bit(3, mem.read_byte(self.HL)));
                Instruction::Ok(opcode,2, 16, "SET 3, (HL)")
            }
            0xdf => {
                let result = self.set_bit(3, self.get_a());
                self.set_a(result);
                Instruction::Ok(opcode,2, 8, "SET 3, A")
            }
            0xe0 => {
                let result = self.set_bit(4, self.get_b());
                self.set_b(result);
                Instruction::Ok(opcode,2, 8, "SET 4, B")
            }
            0xe1 => {
                let result = self.set_bit(4, self.get_c());
                self.set_c(result);
                Instruction::Ok(opcode,2, 8, "SET 4, C")
            }
            0xe2 => {
                let result = self.set_bit(4, self.get_d());
                self.set_d(result);
                Instruction::Ok(opcode,2, 8, "SET 4, D")
            }
            0xe3 => {
                let result = self.set_bit(4, self.get_e());
                self.set_e(result);
                Instruction::Ok(opcode,2, 8, "SET 4, E")
            }
            0xe4 => {
                let result = self.set_bit(4, self.get_h());
                self.set_h(result);
                Instruction::Ok(opcode,2, 8, "SET 4, H")
            }
            0xe5 => {
                let result = self.set_bit(4, self.get_l());
                self.set_l(result);
                Instruction::Ok(opcode,2, 8, "SET 4, L")
            }
            0xe6 => {
                mem.write_byte(self.HL, self.set_bit(4, mem.read_byte(self.HL)));
                Instruction::Ok(opcode,2, 16, "SET 4, (HL)")
            }
            0xe7 => {
                let result = self.set_bit(4, self.get_a());
                self.set_a(result);
                Instruction::Ok(opcode,2, 8, "SET 4, A")
            }
            0xe8 => {
                let result = self.set_bit(5, self.get_b());
                self.set_b(result);
                Instruction::Ok(opcode,2, 8, "SET 5, B")
            }
            0xe9 => {
                let result = self.set_bit(5, self.get_c());
                self.set_c(result);
                Instruction::Ok(opcode,2, 8, "SET 5, C")
            }
            0xea => {
                let result = self.set_bit(5, self.get_d());
                self.set_d(result);
                Instruction::Ok(opcode,2, 8, "SET 5, D")
            }
            0xeb => {
                let result = self.set_bit(5, self.get_e());
                self.set_e(result);
                Instruction::Ok(opcode,2, 8, "SET 5, E")
            }
            0xec => {
                let result = self.set_bit(5, self.get_h());
                self.set_h(result);
                Instruction::Ok(opcode,2, 8, "SET 5, H")
            }
            0xed => {
                let result = self.set_bit(5, self.get_l());
                self.set_l(result);
                Instruction::Ok(opcode,2, 8, "SET 5, L")
            }
            0xee => {
                mem.write_byte(self.HL, self.set_bit(5, mem.read_byte(self.HL)));
                Instruction::Ok(opcode,2, 16, "SET 5, (HL)")
            }
            0xef => {
                let result = self.set_bit(5, self.get_a());
                self.set_a(result);
                Instruction::Ok(opcode,2, 8, "SET 5, A")
            }
            0xf0 => {
                let result = self.set_bit(6, self.get_b());
                self.set_b(result);
                Instruction::Ok(opcode,2, 8, "SET 6, B")
            }
            0xf1 => {
                let result = self.set_bit(6, self.get_c());
                self.set_c(result);
                Instruction::Ok(opcode,2, 8, "SET 6, C")
            }
            0xf2 => {
                let result = self.set_bit(6, self.get_d());
                self.set_d(result);
                Instruction::Ok(opcode,2, 8, "SET 6, D")
            }
            0xf3 => {
                let result = self.set_bit(6, self.get_e());
                self.set_e(result);
                Instruction::Ok(opcode,2, 8, "SET 6, E")
            }
            0xf4 => {
                let result = self.set_bit(6, self.get_h());
                self.set_h(result);
                Instruction::Ok(opcode,2, 8, "SET 6, H")
            }
            0xf5 => {
                let result = self.set_bit(6, self.get_l());
                self.set_l(result);
                Instruction::Ok(opcode,2, 8, "SET 6, L")
            }
            0xf6 => {
                mem.write_byte(self.HL, self.set_bit(6, mem.read_byte(self.HL)));
                Instruction::Ok(opcode,2, 16, "SET 6, (HL)")
            }
            0xf7 => {
                let result = self.set_bit(6, self.get_a());
                self.set_a(result);
                Instruction::Ok(opcode,2, 8, "SET 6, A")
            }
            0xf8 => {
                let result = self.set_bit(7, self.get_b());
                self.set_b(result);
                Instruction::Ok(opcode,2, 8, "SET 7, B")
            }
            0xf9 => {
                let result = self.set_bit(7, self.get_c());
                self.set_c(result);
                Instruction::Ok(opcode,2, 8, "SET 7, C")
            }
            0xfa => {
                let result = self.set_bit(7, self.get_d());
                self.set_d(result);
                Instruction::Ok(opcode,2, 8, "SET 7, D")
            }
            0xfb => {
                let result = self.set_bit(7, self.get_e());
                self.set_e(result);
                Instruction::Ok(opcode,2, 8, "SET 7, E")
            }
            0xfc => {
                let result = self.set_bit(7, self.get_h());
                self.set_h(result);
                Instruction::Ok(opcode,2, 8, "SET 7, H")
            }
            0xfd => {
                let result = self.set_bit(7, self.get_l());
                self.set_l(result);
                Instruction::Ok(opcode,2, 8, "SET 7, L")
            }
            0xfe => {
                mem.write_byte(self.HL, self.set_bit(7, mem.read_byte(self.HL)));
                Instruction::Ok(opcode,2, 16, "SET 7, (HL)")
            }
            0xff => {
                let result = self.set_bit(7, self.get_a());
                self.set_a(result);
                Instruction::Ok(opcode,2, 8, "SET 7, A")
            }
        }
    }
}
