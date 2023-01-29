#[cfg(test)]
mod tests {
    use crate::{
        memory::{gpu::TickMode, Memory, MemoryType},
        video::{self, SCREEN_HEIGHT},
    };

    #[test]
    fn test_modes() {
        let mut memory = Memory::new();
        memory.reset();
        assert_eq!(memory.gpu.mode(), TickMode::HBLANK);
    }

    #[test]
    fn test_draw_bg_tile() {
        let mut target = vec![vec![video::GBColor::White; 8]; 8];
        let t = "
        00000000
        01111110
        20000002
        12222221
        11122233
        01230123
        00000000
        22222222
        ";
        let mut memory = Memory::new();
        
        // 00000000
        memory.write_byte(0x8000, 0b00000000);
        memory.write_byte(0x8001, 0b00000000);

        // 01111110
        memory.write_byte(0x8002, 0b01111110);
        memory.write_byte(0x8003, 0b00000000);

        // 20000002
        memory.write_byte(0x8004, 0b00000000);
        memory.write_byte(0x8005, 0b10000001);

        // 12222221
        memory.write_byte(0x8006, 0b10000001);
        memory.write_byte(0x8007, 0b01111110);

        // 11122233
        memory.write_byte(0x8008, 0b11100011);
        memory.write_byte(0x8009, 0b00011111);

        // 01230123
        memory.write_byte(0x800A, 0b01010101);
        memory.write_byte(0x800B, 0b00110011);

        // 00000000
        memory.write_byte(0x800C, 0);
        memory.write_byte(0x800D, 0);

        // 22222222
        memory.write_byte(0x800E, 0b00000000);
        memory.write_byte(0x800F, 0b11111111);

        let dumped = memory.dump_bg_tiles();
        let tile = dumped[0];

        let vals: Vec<u8> = t
            .chars()
            .filter(|x| x.is_numeric())
            .map(|x| x.to_string().parse::<u8>().unwrap())
            .collect();
        for i in 0..64 {
            let x = i % 8;
            let y = i / 8;

            assert_eq!(video::byte_to_color(vals[i]), tile[y][x], "failed at {i}");
        }
    }

    #[test]
    fn test_bg_tiles_colors() {
        let mut memory = Memory::new();
        memory.write_byte(0x8000, 0xff);
        memory.write_byte(0x8001, 0xff);
        let dumped = memory.dump_bg_tiles();
        for x in 0..8 {
            assert_eq!(dumped[0][0][x], video::GBColor::Black)
        }

        memory.write_byte(0x8000, 0xff);
        memory.write_byte(0x8001, 0x00);
        let dumped = memory.dump_bg_tiles();
        for x in 0..8 {
            assert_eq!(dumped[0][0][x], video::GBColor::LightGray)
        }

        memory.write_byte(0x8000, 0x00);
        memory.write_byte(0x8001, 0xff);
        let dumped = memory.dump_bg_tiles();
        for x in 0..8 {
            assert_eq!(dumped[0][0][x], video::GBColor::DarkGray)
        }
        memory.write_byte(0x8000, 0x00);
        memory.write_byte(0x8001, 0x00);
        let dumped = memory.dump_bg_tiles();
        for x in 0..8 {
            assert_eq!(dumped[0][0][x], video::GBColor::White)
        }
    }
    #[test]
    fn test_bg_tiles_old() {
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
                    assert_eq!(memory.gpu.get_pixel(x, y), video::GBColor::Black);
                } else {
                    assert_eq!(memory.gpu.get_pixel(x, y), video::GBColor::White);
                }
            }
        }
        for y in 0..8 {
            for x in 8..16 {
                assert_eq!(memory.gpu.get_pixel(x, y), video::GBColor::LightGray);
            }
        }
        memory.dump_bg_tiles();
    }
}
