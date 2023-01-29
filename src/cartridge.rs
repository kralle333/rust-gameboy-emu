const KB: usize = 1024;

#[derive(Debug, PartialEq)]
pub enum CartridgeType {
    RomOnly,
    Mbc1,
    Mbc2,
    Invalid = 1000,
}

impl CartridgeType {
    fn from_u32(val: u32) -> CartridgeType {
        match val {
            0 | 8 | 9 => CartridgeType::RomOnly,
            1 | 2 | 3 => CartridgeType::Mbc1,
            5 | 6 => CartridgeType::Mbc2,
            _ => CartridgeType::Invalid,
        }
    }
}

pub struct Cartridge {
    pub cartidge_type: CartridgeType,
    pub rom_bank_size: usize,
    pub ram_bank_size: usize,
}

impl Cartridge {
    pub fn new(data: &Vec<u8>) -> Self {
        let mut cartridge = Cartridge {
            cartidge_type: CartridgeType::Invalid,
            rom_bank_size: 0,
            ram_bank_size: 0,
        };
        cartridge.cartidge_type = CartridgeType::from_u32(data[0x147] as u32);

        cartridge.rom_bank_size = KB
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
        cartridge.ram_bank_size = KB
            * match data[0x149] {
                0 => 0,
                1 => 2,
                2 => 8,
                3 => 32,
                4 => 128,
                _ => unimplemented!(),
            };
        cartridge
    }
}