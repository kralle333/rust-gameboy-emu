#[cfg(test)]
mod tests {
    use crate::memory::{Memory, MemoryType};

    #[test]
    fn test_memory_initialization() {
        let mut memory = Memory::new(); // Create a new instance of the memory struct

        // Verify that the memory is initialized with the correct values
        assert_eq!(memory.read_byte(0x0000), 0x00);
        assert_eq!(memory.read_byte(0x0100), 0xFF);
        assert_eq!(memory.read_byte(0x0200), 0x00);
        // Add more asserts as needed to test all the relevant memory addresses
    }

    #[test]
    fn test_memory_read_write_bytes() {
        let mut memory = Memory::new(); // Create a new instance of the memory struct

        for addr in 0x0000..0xFFFF {
            memory.write_byte(addr, 0xAB); // Set the value at address 0x1234 to 0xAB
            assert_eq!(memory.read_byte(addr), 0xAB); // Verify that the value can be read back correctly
        }
    }
    #[test]
    fn test_memory_read_write_words() {
        let mut memory = Memory::new(); // Create a new instance of the memory struct

        for addr in 0x0000..0xFFFF {
            memory.write_word(addr, 0xAB); // Set the value at address 0x1234 to 0xAB
            assert_eq!(memory.read_word(addr), 0xAB); // Verify that the value can be read back correctly
        }
    }
}
