pub struct Memory {
    rom: [u8; 0x8000],
    work_ram: [u8; 0x2000],
    ext_ram: [u8; 0x2000],
    _high_ram: [u8; 0x7F],
    rom_offset: u8,
    _ram_offset: u8,
}

impl Memory {
    pub fn new() -> Memory {
        todo!()
    }

    pub fn load(&mut self, data: Vec<u8>) {
        self.rom.copy_from_slice(&data);
    }
    pub fn read_byte(&self, addr: u16) -> u8 {
        let addr = addr as usize;
        match addr {
            0x0000..=0x3FFF => self.rom[addr],
            0x4000..=0x7FFF => self.rom[self.rom_offset as usize + (addr & 0x3FFF)],
            0x8000..=0x9FFF => self.rom[addr], // TODO: VIDEO RAM
            0xA000..=0xBFFF => self.ext_ram[addr & 0x1FFF],
            0xC000..=0xDFFF => self.work_ram[addr & 0x1FFF],
            0xE000..=0xFDFF => self.work_ram[(addr - 0x2000) & 0x1FFF],
            _ => panic!("invalid addr {addr}"),
        }
    }

    pub fn read_word(&self, addr: u16) -> u16 {
        let lsn = self.read_byte(addr);
        let msn = self.read_byte(addr + 1);

        msn as u16 | lsn as u16
    }

    pub fn write_byte(&self, _addr: u16, _value: u8) {}

    pub fn write_word(&self, addr: u16, value: u16) {
        self.write_byte(addr, (value & 0xFF) as u8);
        self.write_byte(addr + 1, (value & 0xFF00) as u8);
    }
}
