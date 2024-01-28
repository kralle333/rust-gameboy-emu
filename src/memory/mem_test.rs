#[cfg(test)]
mod tests {
    use crate::memory::{Memory, MemoryType};

    #[test]
    fn test_memory_read_write_bytes() {
        let mut memory = Memory::new();

        // VRAM
        for addr in 0x8000..0xA000 {
            memory.write_byte(addr, 0xAB);
            assert_eq!(memory.read_byte(addr), 0xAB);
        }
        // EXTERNAL RAM
        for addr in 0xA000..0xC000 {
            memory.write_byte(addr, 0xAB);
            assert_eq!(memory.read_byte(addr), 0xAB);
        }

        // INTERNAL RAM
        for addr in 0xC000..0xE000 {
            memory.write_byte(addr, 0xAB);
            assert_eq!(memory.read_byte(addr), 0xAB);
        }
        
    }
    #[test]
    fn test_memory_read_write_words() {
        let mut memory = Memory::new();

        // VRAM
        for addr in 0x8000..0xA000 {
            memory.write_word(addr, 0xABFE);
            assert_eq!(memory.read_word(addr), 0xABFE);
        }
        // EXTERNAL RAM
        for addr in 0xA000..0xC000 {
            memory.write_word(addr, 0xABFE);
            assert_eq!(memory.read_word(addr), 0xABFE);
        }

        // INTERNAL RAM 
        for addr in 0xC000..0xE000 {
            memory.write_word(addr, 0xABFE);
            assert_eq!(memory.read_word(addr), 0xABFE);
        }
    }

    #[test]
    fn test_memory_internal_echo_read() {
        let mut memory = Memory::new();

         memory.write_byte(0xC000,0xFC);
         assert_eq!(memory.read_byte(0xC000),memory.read_byte(0xE000));

        memory.write_word(0xC000,0xFCAB);
        assert_eq!(memory.read_word(0xC000),memory.read_word(0xE000));

    }
    #[test]
    fn test_memory_internal_echo_write() {
        let mut memory = Memory::new();

        memory.write_byte(0xE000,0xFC);
        assert_eq!(memory.read_byte(0xC000),memory.read_byte(0xE000));

        memory.write_word(0xE000,0xFCAB);
        assert_eq!(memory.read_word(0xC000),memory.read_word(0xE000));
    }
}
