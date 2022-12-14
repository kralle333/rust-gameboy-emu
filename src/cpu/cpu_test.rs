#[cfg(test)]
mod tests {
    use rand::Rng;

    use crate::{
        cpu::{self, Cpu, Flag, Register},
        memory::{self, Memory, MemoryType},
    };

    #[derive(Clone, Copy)]
    enum Register16 {
        AF,
        BC,
        DE,
        HL,
        SP,
        PC,
    }

    struct Tester {
        cpu: Cpu,
        mem: Memory,
        rand: rand::rngs::ThreadRng,
    }

    impl Tester {
        fn new() -> Tester {
            Tester {
                cpu: Cpu::new(),
                mem: Memory::new(),
                rand: rand::thread_rng(),
            }
        }
        fn run(&mut self, opcode: u8) {
            self.mem.write_byte(0x0000, opcode);
            self.cpu.PC = 0x0000;
            self.cpu.fetch_decode(&mut self.mem);
        }

        fn assert_eq_reg(&self, a: Register, b: Register) {
            assert_eq!(self.cpu.get_reg(a), self.cpu.get_reg(b))
        }
        fn assert_eq_reg16(&self, a: Register16, b: Register16) {
            assert_eq!(self.get_register16(a), self.get_register16(b))
        }
        fn set_register16(&mut self, reg: Register16, val: u16) {
            match reg {
                Register16::AF => self.cpu.AF = val,
                Register16::BC => self.cpu.BC = val,
                Register16::DE => self.cpu.DE = val,
                Register16::HL => self.cpu.HL = val,
                Register16::SP => self.cpu.SP = val,
                Register16::PC => self.cpu.PC = val,
            }
        }
        fn get_register16(&self, reg: Register16) -> u16 {
            match reg {
                Register16::AF => self.cpu.AF,
                Register16::BC => self.cpu.BC,
                Register16::DE => self.cpu.DE,
                Register16::HL => self.cpu.HL,
                Register16::SP => self.cpu.SP,
                Register16::PC => self.cpu.PC,
            }
        }
        fn test_opcode_reg_8(&mut self, opcode: u8, reg: Register) {
            self.mem.write_byte(0x0000, opcode);
            self.mem.write_word(0x0001, 0x1212);
            self.cpu.PC = 0x0000;
            self.cpu.fetch_decode(&mut self.mem);
            assert_eq!(self.cpu.HL, 0x1212);
        }
        fn test_opcode_reg_16(&mut self, opcode: u8, reg: Register16) {
            let rand_value = self.rand.gen_range(0..=0xFFFF);
            self.mem.write_byte(0x0000, opcode);
            self.mem.write_word(0x0001, rand_value);
            self.cpu.PC = 0x0000;
            self.cpu.fetch_decode(&mut self.mem);
            let target = self.get_register16(reg);
            assert_eq!(target, rand_value);
        }
        fn test_opcode_reg_load_at_addr(&mut self, opcode: u8, target: Register, address: u16) {
            let rand_value = self.rand.gen_range(0..0xFF);
            self.mem.write_byte(address, rand_value);
            self.mem.write_byte(0x0000, opcode);
            self.mem.write_word(0x0001, 0x1212);
            self.cpu.PC = 0x0000;
            self.cpu.fetch_decode(&mut self.mem);
            assert_eq!(self.cpu.get_reg(target), rand_value);
        }
        fn test_opcode_reg_load(&mut self, opcode: u8, target: Register, source: Register) {
            let rand_value = self.rand.gen_range(0..0xFF);
            self.cpu.set_reg(source, rand_value);
            self.run(opcode);
            self.assert_eq_reg(source, target);
        }
        fn test_opcode_reg_load_16(&mut self, opcode: u8, target: Register16, source: Register16) {
            let rand_value = self.rand.gen_range(0..=0xFFFF);
            self.set_register16(source, rand_value);
            let source = self.get_register16(source);
            let target = self.get_register16(target);
            assert_eq!(target, source);
        }
    }

    fn get_registers() -> std::iter::Copied<std::slice::Iter<'static, Register>> {
        let registers = [
            Register::A,
            Register::B,
            Register::C,
            Register::D,
            Register::E,
            Register::H,
            Register::L,
        ]
        .iter()
        .copied();
        registers
    }

    #[test]
    fn test_set_get() {
        let mut c = cpu::Cpu::new();
        c.AF = 0;
        c.BC = 0;
        c.DE = 0;
        c.HL = 0;
        for register in get_registers() {
            for i in 0x00..0xFF {
                c.set_reg(register, i);
                let v = c.get_reg(register);
                assert_eq!(i, v);
            }
        }
    }
    #[test]
    fn test_bits() {
        let mut c = cpu::Cpu::new();

        assert_eq!(c.set_bit(0, 0), 0b00000001);
        assert_eq!(c.set_bit(1, 0), 0b00000010);
        assert_eq!(c.set_bit(2, 0), 0b00000100);
        assert_eq!(c.set_bit(3, 0), 0b00001000);
        assert_eq!(c.set_bit(4, 0), 0b00010000);
        assert_eq!(c.set_bit(5, 0), 0b00100000);
        assert_eq!(c.set_bit(6, 0), 0b01000000);
        assert_eq!(c.set_bit(7, 0), 0b10000000);

        assert_eq!(c.res_bit(0, 0b00000001), 0);
        assert_eq!(c.res_bit(1, 0b00000010), 0);
        assert_eq!(c.res_bit(2, 0b00000100), 0);
        assert_eq!(c.res_bit(3, 0b00001000), 0);
        assert_eq!(c.res_bit(4, 0b00010000), 0);
        assert_eq!(c.res_bit(5, 0b00100001), 1);
        assert_eq!(c.res_bit(6, 0b01000000), 0);
        assert_eq!(c.res_bit(7, 0b10000001), 1);

        assert_eq!(c.swap(0b1100_1111), 0b1111_1100);
        assert_eq!(c.rlc(0b11001111), 0b10011111);
        assert_eq!(c.rrc(0b11001111), 0b11100111);

        c.set_flag(Flag::C, false);
        assert_eq!(c.rl(0b11001111), 0b10011110);
        c.set_flag(Flag::C, false);
        assert_eq!(c.rr(0b11001111), 0b01100111);
        c.set_flag(Flag::C, true);
        assert_eq!(c.rl(0b11001111), 0b10011111);
        c.set_flag(Flag::C, true);
        assert_eq!(c.rr(0b11001111), 0b11100111);
    }

    #[test]
    fn test_rcla() {
        let mut m = memory::Memory::new();
        let mut c = cpu::Cpu::new();
        c.set_a(0b10001000);
        c.set_flag(Flag::C, false);
        let i = c.execute(0x07, &mut m);
        if let Instruction::Ok(_, pc, cycles, info) = i {
            assert_eq!(pc, 1);
            assert_eq!(cycles, 4);
            assert_eq!(info, "RLCA");
        } else {
            assert!(true)
        }
        assert_eq!(c.get_a(), 0b00010001);
        assert_eq!(c.get_flag(Flag::C), true);
    }

    #[test]
    fn test_rra() {
        let mut m = memory::Memory::new();
        let mut c = cpu::Cpu::new();
        c.set_a(0b10001001);
        c.set_flag(Flag::C, true);
        let i = c.execute(0x1f, &mut m);
        match i {
            Instruction::Ok(_, pc, cycles, info) => {
                assert_eq!(pc, 1);
                assert_eq!(cycles, 1);
                assert_eq!(info, "RRA");
            }
            _ => assert!(true),
        }
        assert_eq!(c.get_a(), 0b11000100);
        assert_eq!(c.get_flag(Flag::C), true);
    }

    #[test]
    fn test_flags() {
        let mut c = cpu::Cpu::new();
        c.AF = 0;
        assert_eq!(c.get_flag(Flag::Z), false);
        assert_eq!(c.get_flag(Flag::N), false);
        assert_eq!(c.get_flag(Flag::C), false);
        assert_eq!(c.get_flag(Flag::H), false);

        c.set_flag(Flag::Z, true);
        assert_eq!(c.get_flag(Flag::Z), true);
        assert_eq!(c.get_flag(Flag::N), false);
        assert_eq!(c.get_flag(Flag::C), false);
        assert_eq!(c.get_flag(Flag::H), false);

        c.set_flag(Flag::N, true);
        assert_eq!(c.get_flag(Flag::Z), true);
        assert_eq!(c.get_flag(Flag::N), true);
        assert_eq!(c.get_flag(Flag::C), false);
        assert_eq!(c.get_flag(Flag::H), false);
        c.set_flag(Flag::C, true);
        assert_eq!(c.get_flag(Flag::Z), true);
        assert_eq!(c.get_flag(Flag::N), true);
        assert_eq!(c.get_flag(Flag::C), true);
        assert_eq!(c.get_flag(Flag::H), false);
        c.set_flag(Flag::H, true);
        assert_eq!(c.get_flag(Flag::Z), true);
        assert_eq!(c.get_flag(Flag::N), true);
        assert_eq!(c.get_flag(Flag::C), true);
        assert_eq!(c.get_flag(Flag::H), true);

        assert_eq!(c.AF, 0x00F0);
    }

    #[test]
    fn test_inc_8b() {
        let mut m = memory::Memory::new();
        let mut c = cpu::Cpu::new();
        c.set_e(0xFF);
        c.execute(0x1c, &mut m);
        assert_eq!(c.get_e(), 0);
        assert_eq!(c.get_flag(Flag::H), true);
        assert_eq!(c.get_flag(Flag::Z), true);
        assert_eq!(c.get_flag(Flag::N), false);
    }

    fn test_opcode_reg_8(c: &mut Cpu, m: &mut Memory, opcode: u8, expected_val: u8, reg: Register) {
    }
    #[test]
    fn test_load() {
        let mut t = Tester::new();

        // LD REG d16
        t.test_opcode_reg_16(0x01, Register16::BC);
        t.test_opcode_reg_16(0x11, Register16::DE);
        t.test_opcode_reg_16(0x21, Register16::HL);
        t.test_opcode_reg_16(0x31, Register16::SP);

        t.test_opcode_reg_load(0x40, Register::B, Register::B);
        t.test_opcode_reg_load(0x41, Register::B, Register::C);
        t.test_opcode_reg_load(0x42, Register::B, Register::D);
        t.test_opcode_reg_load(0x43, Register::B, Register::E);
        t.test_opcode_reg_load(0x44, Register::B, Register::H);
        t.test_opcode_reg_load(0x45, Register::B, Register::L);
        t.test_opcode_reg_load_at_addr(0x46, Register::B, t.get_register16(Register16::HL));
        t.test_opcode_reg_load(0x47, Register::B, Register::A);

        t.test_opcode_reg_load(0x48, Register::C, Register::B);
        t.test_opcode_reg_load(0x49, Register::C, Register::C);
        t.test_opcode_reg_load(0x4a, Register::C, Register::D);
        t.test_opcode_reg_load(0x4b, Register::C, Register::E);
        t.test_opcode_reg_load(0x4c, Register::C, Register::H);
        t.test_opcode_reg_load(0x4d, Register::C, Register::L);
        t.test_opcode_reg_load_at_addr(0x4e, Register::C, t.get_register16(Register16::HL));
        t.test_opcode_reg_load(0x4f, Register::C, Register::A);

        t.test_opcode_reg_load(0x50, Register::D, Register::B);
        t.test_opcode_reg_load(0x51, Register::D, Register::C);
        t.test_opcode_reg_load(0x52, Register::D, Register::D);
        t.test_opcode_reg_load(0x53, Register::D, Register::E);
        t.test_opcode_reg_load(0x54, Register::D, Register::H);
        t.test_opcode_reg_load(0x55, Register::D, Register::L);
        t.test_opcode_reg_load_at_addr(0x56, Register::D, t.get_register16(Register16::HL));
        t.test_opcode_reg_load(0x57, Register::D, Register::A);

        t.test_opcode_reg_load(0x58, Register::E, Register::B);
        t.test_opcode_reg_load(0x59, Register::E, Register::C);
        t.test_opcode_reg_load(0x5a, Register::E, Register::D);
        t.test_opcode_reg_load(0x5b, Register::E, Register::E);
        t.test_opcode_reg_load(0x5c, Register::E, Register::H);
        t.test_opcode_reg_load(0x5d, Register::E, Register::L);
        t.test_opcode_reg_load_at_addr(0x5e, Register::E, t.get_register16(Register16::HL));
        t.test_opcode_reg_load(0x5f, Register::E, Register::A);

        t.test_opcode_reg_load(0x60, Register::H, Register::B);
        t.test_opcode_reg_load(0x61, Register::H, Register::C);
        t.test_opcode_reg_load(0x62, Register::H, Register::D);
        t.test_opcode_reg_load(0x63, Register::H, Register::E);
        t.test_opcode_reg_load(0x64, Register::H, Register::H);
        t.test_opcode_reg_load(0x65, Register::H, Register::L);
        t.test_opcode_reg_load_at_addr(0x66, Register::H, t.get_register16(Register16::HL));
        t.test_opcode_reg_load(0x67, Register::H, Register::A);

        t.test_opcode_reg_load(0x68, Register::L, Register::B);
        t.test_opcode_reg_load(0x69, Register::L, Register::C);
        t.test_opcode_reg_load(0x6a, Register::L, Register::D);
        t.test_opcode_reg_load(0x6b, Register::L, Register::E);
        t.test_opcode_reg_load(0x6c, Register::L, Register::H);
        t.test_opcode_reg_load(0x6d, Register::L, Register::L);
        t.test_opcode_reg_load_at_addr(0x6e, Register::L, t.get_register16(Register16::HL));
        t.test_opcode_reg_load(0x6f, Register::L, Register::A);
    }

    #[test]
    fn test_jumps() {
        //0xc3 JP ad8
        let mut t = Tester::new();

        t.mem.write_byte(0x0, 0xc3);
        t.mem.write_word(0x1, 0x0108);
        t.run(0xc3);
        assert_eq!(t.cpu.PC, 0x108);
    }
    #[test]
    fn test_push_pop_stack() {
        let mut c = cpu::Cpu::new();
        let mut mem = memory::Memory::new();

        mem.write_word(c.SP, 0);

        c.push_sp(&mut mem, 0x1234);
        let v = c.pop_sp(&mem);

        assert_eq!(v, 0x1234);
    }
    #[test]
    fn test_hl_plus_minus() {
        let mut t = Tester::new();
        t.cpu.HL = 0x1000;
        t.cpu.set_a(0x66);

        t.run(0x22);
        assert_eq!(0x66, t.mem.read_byte(0x1000));
        assert_eq!(t.cpu.HL, 0x1001);

        t.cpu.HL = 0x2000;
        t.cpu.set_a(0x69);
        t.run(0x32);
        assert_eq!(0x69, t.mem.read_byte(0x2000));
        assert_eq!(t.cpu.HL, 0x1fff);
    }
}
