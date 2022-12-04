use super::MemoryType;

pub struct Mbc {
    rom: [u8; 0x8000],
    rom_offset: i32,
}

impl MemoryType for Mbc {
    fn read_byte(&self,addr: u16)->u8 {
        let addr = addr as usize;
        match addr {
            0x0000..=0x3FFF => self.rom[addr],
            0x4000..=0x7FFF => self.rom[self.rom_offset as usize + (addr & 0x3FFF)],
            _ => panic!("fail")
        }
    }

    fn write_byte(&mut self,addr: u16, val: u8) {
        let addr = addr as usize;
        match addr {
            0x0000..=0x3FFF => self.rom[addr] = val,
            0x4000..=0x7FFF => self.rom[self.rom_offset as usize + (addr & 0x3FFF)] = val,
            _ => panic!("fail")
        }
    }
}

impl Mbc {
    pub fn new() -> Mbc {
        Mbc {
            rom: [0; 0x8000],
            rom_offset: 0,
        }
    }

    pub fn load(&mut self, data: &[u8]) {
        self.rom.copy_from_slice(data);
    }
}
