#[cfg(test)]
mod tests {
    use crate::{
        memory::{
            gpu::{GBColor, TickMode},
            Memory, MemoryType,
        },
        video::SCREEN_HEIGHT,
    };

    #[test]
    fn test_modes() {
        let mut memory = Memory::new();
        memory.reset();
        assert_eq!(memory.gpu.mode(), TickMode::HBLANK);
    }

    #[test]
    fn test_bg_tiles() {
        let mut memory = Memory::new();

        for i in (0x8010..0x8020).step_by(4) {
            memory.write_byte(i, 0xff);
            memory.write_byte(i + 1, 0xff);
            memory.write_byte(i + 2, 0x00);
            memory.write_byte(i + 3, 0x00);
        }
        for i in (0x8020..0x8030).step_by(2) {
            memory.write_byte(i, 0xff);
        }
        memory.write_byte(0xff47, 0b11100100);
        memory.write_byte(0x9800, 0x1);
        memory.write_byte(0x9801, 0x2);
        for i in 0..SCREEN_HEIGHT {
            memory.gpu.write_byte(0xff44, i as u8);
            memory.gpu.render_screen();
        }

        for y in 0..8 {
            for x in 0..8 {
                if y % 2 == 0 {
                    assert_eq!(memory.gpu.get_pixel(x, y), GBColor::Black);
                } else {
                    assert_eq!(memory.gpu.get_pixel(x, y), GBColor::White);
                }
            }
        }
        for y in 0..8 {
            for x in 8..16 {
                assert_eq!(memory.gpu.get_pixel(x, y), GBColor::LightGray);
            }
        }
        memory.dump_bg_tiles();
    }
}
