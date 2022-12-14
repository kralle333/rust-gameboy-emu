#[cfg(test)]
mod tests {
    use crate::memory::{Memory, MemoryType};

    #[test]
    fn test_memory_read_write_bytes() {
        let mut memory = Memory::new();

        for addr in 0x0000..0xfea0 {
            memory.write_byte(addr, 0xAB);
            assert_eq!(memory.read_byte(addr), 0xAB);
        }
        for addr in 0xff00..0xff07 {
            if addr != 0xff03 && addr != 0xff04 {
                memory.write_byte(addr, 0xAB);
                assert_eq!(memory.read_byte(addr), 0xAB);
            }
        }

        // TODO: sound registers
        for addr in 0xff0f..0xffff {
            memory.write_byte(addr, 0xAB);
            assert_eq!(memory.read_byte(addr), 0xAB);
        }
        memory.write_byte(0xff04, 0xAB);
        assert_eq!(memory.read_byte(0xff04), 0);
    }
    #[test]
    fn test_memory_read_write_words() {
        let mut memory = Memory::new(); // Create a new instance of the memory struct

        for addr in 0x0000..0xfea0 {
            memory.write_word(addr, 0xAB); // Set the value at address 0x1234 to 0xAB
            assert_eq!(memory.read_word(addr), 0xAB); // Verify that the value can be read back correctly
        }
    }
}
