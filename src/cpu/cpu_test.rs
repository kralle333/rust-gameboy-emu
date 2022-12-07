#[cfg(test)]
mod tests {
    use crate::{
        cpu::{self, helpers::Instruction, Flag, Register},
        memory::{self, MemoryType},
    };
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

    #[test]
    fn test_jumps() {
        //0xc3 JP ad8
        let mut c = cpu::Cpu::new();
        let mut mem = memory::Memory::new();

        mem.write_byte(0x100, 0xc3);
        mem.write_word(0x101, 0x0108);

        c.fetch_decode(&mut mem);
        c.fetch_decode(&mut mem);

        assert_eq!(c.PC, 0x108);
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
}
