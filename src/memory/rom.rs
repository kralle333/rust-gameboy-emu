use crate::cartridge::{Cartridge, CartridgeType};

use super::MemoryType;

#[derive(PartialEq, Debug)]
enum MbcMode {
    None,
    Mbc1_16mbRom8kbRam,
    Mbc1_4mbRom32kbRam,
    Invalid,
}

pub struct Rom {
    rom: Vec<u8>,
    external_ram: [u8; 0x2000],
    internal_ram: [u8; 0x2000],
    high_ram: [u8; 0x7f],

    //Access
    rom_offset: usize,
    ram_offset: usize,

    //Info
    mbc_mode: MbcMode,
    ram_enabled: bool,

    //Debug
    log_bank_changes: bool,
}

impl MemoryType for Rom {
    fn read_byte(&self, addr: u16) -> u8 {
        let addr = addr as usize;
        match addr {
            0x0000..=0x3fff => self.rom[addr],
            0x4000..=0x7fff => self.rom[(addr & 0x3fff) + self.rom_offset as usize],
            0xa000..=0xbfff => self.external_ram[(addr & 0x1fff) + self.ram_offset],
            0xc000..=0xdfff => self.internal_ram[addr & 0x1fff],
            0xe000..=0xfeff => { self.internal_ram[(addr - 0x2000) & 0x1fff] } // echo
            0xff00..=0xfffe => self.high_ram[addr & 0x7f],
            _ => panic!("fail"),
        }
    }

    fn write_byte(&mut self, addr: u16, val: u8) {
        match addr {
            0x0000..=0x7fff => match self.mbc_mode {
                MbcMode::None => {}
                MbcMode::Mbc1_16mbRom8kbRam | MbcMode::Mbc1_4mbRom32kbRam => {
                    self.write_mbc1(addr, val);
                }
                MbcMode::Invalid => todo!(),
            },
            0xa000..=0xbfff => self.external_ram[(addr & 0x1fff) as usize + self.ram_offset] = val,
            0xc000..=0xdfff => self.internal_ram[addr as usize & 0x1fff] = val,
            0xe000..=0xfdff => self.internal_ram[(addr as usize - 0x2000) & 0x1fff] = val, // echo
            0xff00..=0xfffe => self.high_ram[addr as usize & 0x7f] = val,
            _ => panic!("fail"),
        }
    }
}

impl Rom {
    pub fn new() -> Self {
        Self {
            rom: Vec::new(),
            rom_offset: 0x4000,
            ram_offset: 0,
            internal_ram: [0; 0x2000],
            external_ram: [0; 0x2000],
            high_ram: [0; 0x7f],
            mbc_mode: MbcMode::Invalid,
            ram_enabled: false,
            log_bank_changes:false,
        }
    }
    fn write_mbc1(&mut self, addr: u16, val: u8) {
        match addr {
            0x0000..=0x1fff => {
                self.ram_enabled = (val & 0xff) == 0x0a;
            }
            0x2000..=0x3fff => {
                let mut rom_bank = val & 0x3;
                if rom_bank == 0 {
                    rom_bank = 1;
                }
                let new_offset = match self.mbc_mode {
                    MbcMode::None => 0x4000,
                    MbcMode::Mbc1_16mbRom8kbRam => (rom_bank as usize) * 0x4000,
                    MbcMode::Mbc1_4mbRom32kbRam => (rom_bank as usize) * 4 * 0x1000,
                    _ => 0x4000,
                };
                if self.rom_offset != new_offset {
                    self.rom_offset = new_offset;
                    if self.log_bank_changes {
                        println!("switched to rom bank {rom_bank}");
                    }
                }
            }
            0x4000..=0x5fff => {
                if self.mbc_mode == MbcMode::Mbc1_4mbRom32kbRam {
                    let ram_bank = val & 0x3;
                    self.ram_offset = ram_bank as usize * 0x2000;
                    if self.log_bank_changes {
                        println!("switched to ram bank {ram_bank}");
                    }
                }
            }
            0x6000..=0x7fff => {
                let new_mode =
                    if (val & 1) == 0 {
                        MbcMode::Mbc1_16mbRom8kbRam
                    } else {
                        MbcMode::Mbc1_4mbRom32kbRam
                    };
                if self.mbc_mode != new_mode {
                    if self.log_bank_changes {
                        //println!("switched mbc1 mode: {:?}", new_mode);
                    }
                    self.mbc_mode = new_mode;
                }
            }
            _ => panic!(),
        }
    }

    pub fn load(&mut self, data: &[u8], cartridge_info: &Cartridge) {
        self.rom = vec![0; cartridge_info.rom_size];
        self.rom.copy_from_slice(&data);

        match cartridge_info.cartidge_type {
            CartridgeType::RomOnly => self.mbc_mode = MbcMode::None,
            CartridgeType::Mbc1 => self.mbc_mode = MbcMode::Mbc1_16mbRom8kbRam,
            CartridgeType::Mbc2 => todo!(),
            CartridgeType::Mbc3 => todo!(),
            CartridgeType::Mbc5 => todo!(),
            CartridgeType::Invalid => todo!(),
        }
    }
}
