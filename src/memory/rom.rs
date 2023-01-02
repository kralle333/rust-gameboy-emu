use super::MemoryType;

const KB: usize = 1024;
const MB: usize = KB * 1024;

#[derive(PartialEq)]
enum MbcMode {
    Mbc1_16mbRom8kbRam,
    Mbc1_4mbRom32kbRam,
    Invalid,
}
#[derive(Debug)]
enum CartridgeType {
    RomOnly = 0,
    RomMbc1 = 1,
    RomMbc1Ram = 2,
    RomMbc1RamBatt = 3,
    RomMbc2 = 5,
    RomMbc2Batt = 6,
    RomRam = 8,
    RomRamBatt = 9,
    Invalid = 1000,
}

impl CartridgeType {
    fn from_u32(val: u32) -> CartridgeType {
        match val {
            0 => CartridgeType::RomOnly,
            1 => CartridgeType::RomMbc1,
            2 => CartridgeType::RomMbc1Ram,
            3 => CartridgeType::RomMbc1RamBatt,
            5 => CartridgeType::RomMbc2,
            6 => CartridgeType::RomMbc2Batt,
            8 => CartridgeType::RomRam,
            9 => CartridgeType::RomRamBatt,
            _ => CartridgeType::Invalid,
        }
    }
}

pub struct Rom {
    rom: Vec<u8>,
    external_ram: Vec<u8>,
    internal_ram: [u8; 0x2000],

    //Access
    rom_offset: usize,
    ram_offset: usize,
    mbc_mode: MbcMode,
    ram_enabled: bool,

    //Info
    cartidge_type: CartridgeType,
    rom_bank_size: usize,
    ram_bank_size: usize,
}

impl MemoryType for Rom {
    fn read_byte(&self, addr: u16) -> u8 {
        let addr = addr as usize;
        match addr {
            0x0000..=0x3fff => self.rom[addr],
            0x4000..=0x7fff => self.rom[(addr & 0x3fff) + self.rom_offset as usize],
            0xa000..=0xbfff => self.external_ram[(addr & 0x1fff) + self.ram_offset],
            0xc000..=0xdfff => self.internal_ram[addr & 0x1fff],
            0xe000..=0xfdff => self.internal_ram[(addr - 0x2000) & 0x1fff], // echo
            _ => panic!("fail"),
        }
    }

    fn write_byte(&mut self, addr: u16, val: u8) {
        let addr = addr as usize;
        match addr {
            0x0000..=0x7fff => {
                // Select some stuff
                match self.cartidge_type {
                    CartridgeType::RomMbc1
                    | CartridgeType::RomMbc1Ram
                    | CartridgeType::RomMbc1RamBatt => match addr {
                        0x0000..=0x1fff => {
                            self.ram_enabled = (addr & 0xff) == 0x0a;
                        }
                        0x2000..=0x3fff => {
                            let mut rom_bank = addr & 0x3;
                            if rom_bank == 0 {
                                rom_bank = 1;
                            }
                            self.rom_offset = match self.mbc_mode {
                                MbcMode::Mbc1_16mbRom8kbRam => (rom_bank) * 0x4000,
                                MbcMode::Mbc1_4mbRom32kbRam => (rom_bank) * 4 * 0x1000,
                                _ => unimplemented!(),
                            };
                            println!("switched to rom bank {rom_bank}");
                        }
                        0x4000..=0x5fff => {
                            if self.mbc_mode == MbcMode::Mbc1_4mbRom32kbRam {
                                let ram_bank = addr & 0x3;
                                self.ram_offset = ram_bank * 0x2000;
                                println!("switced to ram bank {ram_bank}");
                            }
                        }
                        0x6000..=0x7fff => {
                            self.mbc_mode = if (addr as u8 & 1) == 1 {
                                MbcMode::Mbc1_16mbRom8kbRam
                            } else {
                                MbcMode::Mbc1_4mbRom32kbRam
                            };
                        }
                        _ => panic!(),
                    },
                    _ => unimplemented!(),
                }
            }
            0xa000..=0xbfff => self.external_ram[addr & 0x1fff] = val,
            0xc000..=0xdfff => self.internal_ram[addr & 0x1fff] = val,
            0xe000..=0xfdff => self.internal_ram[(addr - 0x2000) & 0x1fff] = val, // echo
            _ => panic!("fail"),
        }
    }
}

impl Rom {
    pub fn new() -> Self {
        Self {
            rom: Vec::new(),
            external_ram: Vec::new(),
            rom_offset: 0,
            ram_offset: 0,
            rom_bank_size: 0,
            ram_bank_size: 0,
            cartidge_type: CartridgeType::Invalid,
            internal_ram: [0; 0x2000],
            mbc_mode: MbcMode::Invalid,
            ram_enabled: false,
        }
    }

    pub fn load(&mut self, data: &[u8]) {
        self.cartidge_type = CartridgeType::from_u32(data[0x147] as u32);

        match self.cartidge_type {
            CartridgeType::RomOnly => {}
            CartridgeType::RomMbc1 | CartridgeType::RomMbc1Ram | CartridgeType::RomMbc1RamBatt => {
                self.mbc_mode = MbcMode::Mbc1_16mbRom8kbRam
            }
            CartridgeType::RomMbc2 => todo!(),
            CartridgeType::RomMbc2Batt => todo!(),
            CartridgeType::RomRam => todo!(),
            CartridgeType::RomRamBatt => todo!(),
            CartridgeType::Invalid => todo!(),
        }

        self.rom_bank_size = KB
            * match data[0x148] {
                0 => 32,
                1 => 64,
                2 => 128,
                3 => 256,
                4 => 512,
                5 => KB,
                6 => KB * 2,
                _ => unimplemented!(),
            };
        self.rom = vec![0; self.rom_bank_size];
        self.ram_bank_size = KB
            * match data[0x149] {
                0 => 0,
                1 => 2,
                2 => 8,
                3 => 32,
                4 => 128,
                _ => unimplemented!(),
            };
        self.external_ram = vec![0; self.ram_bank_size];

        self.rom.copy_from_slice(&data);

        println!(
            "Success: Rom Size {0}KB Ram {1}KB, Catridge {2:?}",
            self.rom_bank_size / KB,
            self.ram_bank_size / KB,
            self.cartidge_type
        );
    }
}