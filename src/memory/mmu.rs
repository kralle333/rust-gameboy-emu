use super::MemoryType;



pub struct Mmu{
    work_ram: [u8; 0x2000],
    ext_ram: [u8; 0x2000],
    _high_ram: [u8; 0x7f],
    _ram_offset: u8,
}

impl MemoryType for Mmu {
    fn read_byte(&self,addr: u16)->u8 {
        let addr = addr as usize;
        match addr {
            0xa000..=0xbfff => self.ext_ram[addr & 0x1fff],
            0xc000..=0xdfff => self.work_ram[addr & 0x1fff],
            0xe000..=0xfdff => self.work_ram[(addr - 0x2000) & 0x1fff], // echo
            0xff80..=0xfffe => self._high_ram[addr & 0x7e],
            _ => panic!("mmu fail")
        }
    }

    fn write_byte(&mut self,addr: u16, val: u8) {
        let addr = addr as usize;
        match addr {
            0xa000..=0xbfff => self.ext_ram[addr & 0x1fff] = val,
            0xc000..=0xdfff => self.work_ram[addr & 0x1fff] = val,
            0xe000..=0xfdff => self.work_ram[(addr - 0x2000) & 0x1fff] = val,
            0xff80..=0xfffe => self._high_ram[addr & 0x7e] = val,
            _ => panic!("mmu fail")
        }
    }
}
impl Mmu {
    pub fn new() -> Mmu{
        Mmu { 
            work_ram: [0; 0x2000],
            ext_ram: [0; 0x2000],
            _high_ram: [0; 0x7f],
            _ram_offset: 0,
         }
    }
}