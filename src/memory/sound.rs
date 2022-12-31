use super::MemoryType;

struct SoundRegister {
    sweep: u8,
    sound_len: u8,
    envelope: u8,
    freq_lo: u8,
    freq_hi: u8,
}

impl SoundRegister {
    fn new() -> SoundRegister {
        SoundRegister {
            sweep: 0,
            sound_len: 0,
            envelope: 0,
            freq_lo: 0,
            freq_hi: 0,
        }
    }
}

struct SoundRegister3 {
    on_off: bool,
    sound_len: u8,
    output_lvl: u8,
    freq_lo: u8,
    freq_hi: u8,
}

impl SoundRegister3 {
    fn new() -> SoundRegister3 {
        SoundRegister3 {
            on_off: false,
            sound_len: 0,
            output_lvl: 0,
            freq_lo: 0,
            freq_hi: 0,
        }
    }
}

struct SoundRegister4 {
    sound_len: u8,
    envelope: u8,
    poly_counter: u8,
    counter_consec: u8,
}

impl SoundRegister4 {
    fn new() -> SoundRegister4 {
        SoundRegister4 {
            sound_len: 0,
            envelope: 0,
            poly_counter: 0,
            counter_consec: 0,
        }
    }
}

pub struct Sound {
    mode_1_reg: SoundRegister,
    mode_2_reg: SoundRegister,
    mode_3_reg: SoundRegister3,
    mode_4_reg: SoundRegister4,
    channel_control: u8,
    output_terminal_selection: u8,
    on_off: u8,
    wave_pattern_ram: [u8; 0xF],
}
impl Sound {
    pub(crate) fn new() -> Sound {
        Sound {
            mode_1_reg: SoundRegister::new(),
            mode_2_reg: SoundRegister::new(),
            mode_3_reg: SoundRegister3::new(),
            mode_4_reg: SoundRegister4::new(),
            channel_control: 0,
            output_terminal_selection: 0,
            on_off: 0xf1,
            wave_pattern_ram: [0; 0xF],
        }
    }
}

impl MemoryType for Sound {
    fn read_byte(&self, addr: u16) -> u8 {
        match addr {
            0xff10 => self.mode_1_reg.sweep,
            0xff11 => self.mode_1_reg.sound_len & 0xc0,
            0xff12 => self.mode_1_reg.envelope,
            0xff13 => self.mode_1_reg.freq_lo,
            0xff14 => self.mode_1_reg.freq_hi & 0x40,
            0xff16 => self.mode_2_reg.sound_len & 0xc0,
            0xff17 => self.mode_2_reg.envelope,
            0xff18 => self.mode_2_reg.freq_lo,
            0xff19 => self.mode_2_reg.freq_hi & 0x40,
            0xff1a => (self.mode_3_reg.on_off as u8) << 7,
            0xff1b => self.mode_3_reg.sound_len,
            0xff1c => self.mode_3_reg.output_lvl & 0x60,
            0xff1d => self.mode_3_reg.freq_lo,
            0xff1e => self.mode_3_reg.freq_hi & 0x40,
            0xff20 => self.mode_4_reg.sound_len,
            0xff21 => self.mode_4_reg.envelope,
            0xff22 => self.mode_4_reg.poly_counter,
            0xff23 => self.mode_4_reg.counter_consec & 0x40,
            0xff24 => self.channel_control,
            0xff25 => self.output_terminal_selection,
            0xff26 => self.on_off & 0xF0, // Bits 0 - 3 of this register are meant to be status bits to be read.
            0xff30..=0xff3f => self.wave_pattern_ram[(addr & 0xF) as usize],
            _ => {
                println!("[Sound]: read from unused addresses of sound");
                0
            }
        }
    }

    fn write_byte(&mut self, addr: u16, val: u8) {
        match addr {
            0xff10 => self.mode_1_reg.sweep = val,
            0xff11 => self.mode_1_reg.sound_len = val,
            0xff12 => self.mode_1_reg.envelope = val,
            0xff13 => self.mode_1_reg.freq_lo = val,
            0xff14 => self.mode_1_reg.freq_hi = val,
            0xff16 => self.mode_2_reg.sound_len = val,
            0xff17 => self.mode_2_reg.envelope = val,
            0xff18 => self.mode_2_reg.freq_lo = val,
            0xff19 => self.mode_2_reg.freq_hi = val,
            0xff1a => self.mode_3_reg.on_off = (val & (1 << 7)) > 0,
            0xff1b => self.mode_3_reg.sound_len = val,
            0xff1c => self.mode_3_reg.output_lvl = val,
            0xff1d => self.mode_3_reg.freq_lo = val,
            0xff1e => self.mode_3_reg.freq_hi = val,
            0xff20 => self.mode_4_reg.sound_len = val,
            0xff21 => self.mode_4_reg.envelope = val,
            0xff22 => self.mode_4_reg.poly_counter = val,
            0xff23 => self.mode_4_reg.counter_consec = val,
            0xff24 => self.channel_control = val,
            0xff25 => self.output_terminal_selection = val,
            0xff26 => self.on_off = val & 0xF0, // Bits 0 - 3 of this register are meant to be status bits to be read.
            0xff30..=0xff3f => self.wave_pattern_ram[(addr & 0xE) as usize] = val,
            _ => println!("[Sound]: wrote to unused addresses of sound"),
        }
    }
}
