use std::collections::HashMap;
use sdl2::{rect::Rect, render::Canvas, video::Window};

use super::MemoryType;
use crate::video::{self, GBColor, SCREEN_HEIGHT, SCREEN_WIDTH};

#[derive(Debug, PartialEq)]
pub enum TickMode {
    HBLANK = 0b00,
    VBLANK = 0b01,
    OAM = 0b10,
    OAMVRAM = 0b11,
}


impl TickMode {
    fn from_val(val: u8) -> TickMode {
        match val {
            0b00 => TickMode::HBLANK,
            0b01 => TickMode::VBLANK,
            0b10 => TickMode::OAM,
            0b11 => TickMode::OAMVRAM,
            _ => TickMode::HBLANK,
        }
    }
}

type Tile16 = [[GBColor; 8]; 8];

fn make_tile16() -> Tile16 {
    [[GBColor::White; 8]; 8]
}

#[derive(Clone, Copy)]
struct ObjData {
    x: u8,
    y: u8,
    pattern_num: u8,
    priority: bool,
    y_flip: bool,
    x_flip: bool,
    pal_num: bool,
}

impl ObjData {
    fn new() -> Self {
        Self {
            x: 0,
            y: 0,
            pattern_num: 0,
            priority: false,
            y_flip: false,
            x_flip: false,
            pal_num: false,
        }
    }
}

#[derive(Debug)]
enum PaletteType {
    Background,
    Object0,
    Object1,
}

pub struct Gpu {
    vram: [u8; 0x2000],
    objects: [ObjData; 40],
    oam: [u8; 0xA0],
    bg_tiles: [Tile16; 384],
    clock: u32,
    can_draw: bool,
    //Video registers
    //FF40
    lcdc: u8,
    //FF41
    lcdc_stat: u8,
    //FF42
    scroll_x: u8,
    //FF43
    scroll_y: u8,
    //FF44
    vert_line: u8,
    //FF45
    vert_line_cp: u8,
    //FF46
    dma_write_addr: u8,
    //FF4A
    window_y: u8,
    //FF4B
    window_x: u8,
    //FF47,
    bg_palette: u8,
    //FF48
    obj_palette0: u8,
    //FF49
    obj_palette1: u8,
    //FF47
    background_palette: [GBColor; 4],
    //FF48
    object_palette0: [GBColor; 4],
    //FF49
    object_palette1: [GBColor; 4],
    pixels: [GBColor; (video::SCREEN_WIDTH * video::SCREEN_HEIGHT) as usize],
}

impl MemoryType for Gpu {
    fn read_byte(&self, addr: u16) -> u8 {
        match addr {
            0x8000..=0x9fff => self.vram[(addr & 0x1fff) as usize],
            0xfe00..=0xfea0 => self.oam[(addr & 160) as usize],
            0xff40..=0xff4b => match addr & 0x00FF {
                0x40 => self.lcdc,
                0x41 => self.lcdc_stat,
                0x42 => self.scroll_y,
                0x43 => self.scroll_x,
                0x44 => self.vert_line,
                0x45 => self.vert_line_cp,
                0x47 => self.bg_palette,
                0x48 => self.obj_palette0,
                0x49 => self.obj_palette1,
                0x4a => self.window_x,
                0x4b => self.window_y,
                _ => panic!("video flags"),
            },
            _ => panic!("video"),
        }
    }

    fn write_byte(&mut self, addr: u16, val: u8) {
        match addr {
            0x8000..=0x97ff => {
                self.vram[(addr & 0x1fff) as usize] = val;
                self.update_tile_data(addr);
            }
            0x9800..=0x9fff => {
                self.vram[(addr & 0x1fff) as usize] = val;
            }
            0xfe00..=0xfea0 => {
                self.oam[(addr & 160) as usize] = val;
                self.update_object_data(addr, val);
            }
            0xff40..=0xff4b => match addr & 0x00FF {
                0x40 => {
                    self.lcdc = val;
                    //println!("lcdc: {val:b}");
                }
                0x41 => {
                    self.lcdc_stat = val;
                    self.set_mode(TickMode::from_val(val & 0x3));
                }
                0x42 => self.scroll_y = val,
                0x43 => self.scroll_x = val,
                0x44 => {
                    println!("vertline read only!")
                }
                0x45 => {
                    self.vert_line_cp = val;
                    println!("LYC: ${val:X}")
                }
                0x46 => self.dma_write_addr = val,
                0x47 => {
                    self.bg_palette = val;
                    self.update_palette(PaletteType::Background, val);
                }
                0x48 => {
                    self.obj_palette0 = val;
                    self.update_palette(PaletteType::Object0, val)
                }
                0x49 => {
                    self.obj_palette1 = val;
                    self.update_palette(PaletteType::Object1, val)
                }
                0x4a => self.window_y = val,
                0x4b => self.window_x = val,
                _ => panic!("video flags"),
            },
            _ => panic!("video addr {addr}"),
        }
    }
}

impl Gpu {
    pub fn new() -> Gpu {
        Gpu {
            objects: [ObjData::new(); 40],
            vram: [0; 0x2000],
            oam: [0; 0xA0],
            bg_tiles: [make_tile16(); 384],
            can_draw: false,
            lcdc: 0x91,
            lcdc_stat: 0,
            scroll_x: 0,
            scroll_y: 0,
            vert_line: 0,
            vert_line_cp: 0,
            window_y: 0,
            window_x: 0,
            bg_palette: 0,
            obj_palette0: 0,
            obj_palette1: 0,
            dma_write_addr: 0,
            pixels: [GBColor::White; (video::SCREEN_WIDTH * video::SCREEN_HEIGHT) as usize],
            background_palette: [GBColor::White; 4],
            object_palette0: [GBColor::White; 4],
            object_palette1: [GBColor::White; 4],
            clock: 0,
        }
    }

    fn is_bit_set(val: u8, bit: u8) -> bool {
        return (val & (1 << bit)) == (1 << bit);
    }

    pub(crate) fn should_display_background(&self) -> bool {
        return Self::is_bit_set(self.lcdc, 0);
    }
    pub(crate) fn use_zero_as_window_solid(&self) -> bool {
        return Self::is_bit_set(self.lcdc, 1);
    }

    pub(crate) fn use_8x16_sprites(&self) -> bool {
        return Self::is_bit_set(self.lcdc, 2);
    }

    // 1: 9C00-9FFF 0: 9800-9BFF
    pub(crate) fn background_tile_table_address(&self) -> bool {
        return Self::is_bit_set(self.lcdc, 3);
    }

    // 1: 8000-8FFF 0: 8800-97FF
    pub(crate) fn tile_pattern_table_address(&self) -> bool {
        return Self::is_bit_set(self.lcdc, 4);
    }
    pub(crate) fn should_draw_window(&self) -> bool {
        return Self::is_bit_set(self.lcdc, 5);
    }

    // 1: 9C00-9FFF 0: 9800-9BFF
    pub(crate) fn window_tile_table_address(&self) -> bool {
        return Self::is_bit_set(self.lcdc, 6);
    }

    pub(crate) fn lcd_operation(&self) -> bool {
        return Self::is_bit_set(self.lcdc, 7);
    }

    pub(crate) fn get_pixel(&self, x: usize, y: usize) -> GBColor {
        self.pixels[x + y * SCREEN_WIDTH]
    }
    fn set_mode(&mut self, val: TickMode) {
        let old = self.mode();
        if old != val {
            //println!("entering mode {:?}", val);
            self.clock = 0;
            self.lcdc_stat = (self.lcdc_stat & !0x3) | (val as u8);
        }
    }
    pub fn mode(&self) -> TickMode {
        TickMode::from_val(self.lcdc_stat & 0x3)
    }
    fn update_tile_data(&mut self, addr: u16) {
        let mut addr = addr & 0x1fff;
        if addr & 1 == 1 {
            addr -= 1;
        } //Because each line is represented as 2 lines, start with the first one
        let tile = addr / 16; // shift 4=div 16 - Each tile is 16 byte - 256x2 tiles

        let y = (addr >> 1) & 7;

        let mut resulting_color: u8;
        for x in 0..8 {
            let sx = 1 << (7 - x);
            resulting_color = 0;
            if (self.vram[addr as usize] & sx) > 0 {
                resulting_color += 1;
            }
            if (self.vram[(addr + 1) as usize] & sx) > 0 {
                resulting_color += 2;
            }

            self.bg_tiles[tile as usize][y as usize][x as usize] =
                video::byte_to_color(resulting_color);
        }
    }
    fn update_object_data(&mut self, addr: u16, val: u8) {
        let obj = ((addr & 0x00FF) >> 2) as usize; //Get F9 bits, divide with 4 to get obj correct id
        match addr & 3 {
            0 => self.objects[obj].y = val,
            1 => self.objects[obj].x = val,
            2 => self.objects[obj].pattern_num = val,
            3 => {
                self.objects[obj].priority = (val & (1 << 7)) > 0;
                self.objects[obj].y_flip = (val & (1 << 6)) > 0;
                self.objects[obj].x_flip = (val & (1 << 5)) > 0;
                self.objects[obj].pal_num = (val & (1 << 4)) > 0;
            }
            _ => panic!("impossible"),
        }
    }
    fn update_palette(&mut self, pal: PaletteType, val: u8) {
        let mut palette = [GBColor::White; 4];
        //println!("updating palette: {0:?} to {1:#04b}", pal, val);
        for i in 0..4 {
            let new_color = (val >> (i * 2)) & 0b11;
            palette[i] = video::byte_to_color(new_color);
        }
        match pal {
            PaletteType::Background => self.background_palette = palette,
            PaletteType::Object0 => self.object_palette0 = palette,
            PaletteType::Object1 => self.object_palette1 = palette,
        };
    }

    pub fn draw(&mut self, canvas: &mut Canvas<Window>) -> bool {
        if !self.can_draw || !self.lcd_operation() {
            return false;
        }
        self.can_draw = false;
        let scheme = &video::ColorScheme::BlackWhite;
        for i in 0..SCREEN_WIDTH * SCREEN_HEIGHT {
            let color = video::get_color(&self.pixels[i], scheme);
            canvas.set_draw_color(color);
            let x = (i % SCREEN_WIDTH) * video::PIXEL_SIZE;
            let y = (i / SCREEN_WIDTH) * video::PIXEL_SIZE;
            match canvas.fill_rect(Rect::new(
                x as i32,
                y as i32,
                video::PIXEL_SIZE as u32,
                video::PIXEL_SIZE as u32,
            )) {
                Ok(_) => {}
                Err(err) => panic!("{err}"),
            }
        }
        return true;
    }

    // returns true if LYC=LY interrupt is triggered
    fn inc_vert_line(&mut self) -> bool {
        self.vert_line += 1;
        if self.vert_line == self.vert_line_cp {
            self.lcdc_stat |= 0x4; //LYC = LCDC LY
            if (Self::is_bit_set(self.lcdc_stat, 6)) {
                return true;
            }
        } else {
            self.lcdc_stat ^= 1 << 0x4; // LYC not equal to LCDC LY
        }
        return false;
    }

    pub fn tick(&mut self, clock_t: u32) -> u8 {
        let mut interrupts: u8 = 0;
        self.clock += clock_t;

        match self.mode() {
            //OAM read
            TickMode::OAM if self.clock >= 80 => {
                self.set_mode(TickMode::OAMVRAM);
            }
            //OAM and VRAM reading
            TickMode::OAMVRAM if self.clock >= 172 => {
                self.set_mode(TickMode::HBLANK);
                self.render_screen();
            }
            //HBlank
            TickMode::HBLANK if self.clock >= 204 => {
                if (self.inc_vert_line()) {
                    interrupts |= 0x2;
                }
                if self.vert_line >= 144 {
                    self.set_mode(TickMode::VBLANK);
                    self.can_draw = true;
                    interrupts |= 0x1;
                    //WriteTileDataToFile("../tiledata.txt");
                    //WriteTileMapToFile("../tilemap.txt");
                } else {
                    self.set_mode(TickMode::OAM);
                }
            }
            //VBlank
            TickMode::VBLANK if self.clock >= 114 => {
                if (self.inc_vert_line()) {
                    interrupts |= 0x2;
                }
                self.clock = 0;
                if self.vert_line >= 153 {
                    self.set_mode(TickMode::OAM);
                    self.vert_line = 0;
                }
            }
            TickMode::HBLANK | TickMode::VBLANK | TickMode::OAM | TickMode::OAMVRAM => {}
        }

        interrupts
    }

    pub fn render_screen(&mut self) {
        self.render_bg();
        self.ren();
        //self.render_objects();
    }
    fn render_bg(&mut self) {
        if !self.should_display_background() {
            return;
        }

        // VRAM offset for the tile map
        let mut map_offs: u16 = 0x1800;
        if self.background_tile_table_address() {
            map_offs = 0x1C00
        }

        // Which line of tiles to use in the map
        map_offs += (((self.vert_line.wrapping_add(self.scroll_y)) >> 3) as u16) << 5;

        // Which tile to start with in the map line
        let mut line_offset: u16 = self.scroll_x as u16 >> 3;

        // Which line of pixels to use in the tiles
        let y = ((self.vert_line.wrapping_add(self.scroll_y)) & 7) as usize;
        // Where in the tileline to start
        let mut x = (self.scroll_x & 7) as usize;

        // Where to render on the canvas
        let mut canvasoffs: u32 = (self.vert_line as usize * video::SCREEN_WIDTH) as u32;

        // Read tile index from the background map
        let mut tile = self.vram[(map_offs + line_offset) as usize] as u16;

        // If the tile data set in use is #1, the
        // indices are signed; calculate a real tile offset
        if self.tile_pattern_table_address() && tile < 128 {
            tile += 256;
        }
        for _ in 0..SCREEN_WIDTH {
            // Re-map the tile pixel through the palette
            let pal_color = self.background_palette[self.bg_tiles[tile as usize][y][x] as usize];

            // Plot the pixel to canvas…
            self.pixels[canvasoffs as usize] = pal_color;

            canvasoffs += 1;

            // When this tile ends, read another
            x += 1;
            if x == 8 {
                x = 0;
                line_offset = (line_offset + 1) & 31;
                tile = self.vram[(map_offs + line_offset) as usize] as u16;
                if !self.tile_pattern_table_address() && tile < 128 {
                    tile += 256;
                }
            }
        }
    }
    fn ren(&mut self) {
        if !self.should_draw_window() {
            return;
        }
        let mut tilemap_addr_start: u16 = 0x1800;
        if self.window_tile_table_address() {
            tilemap_addr_start = 0x1C00
        }
        if self.window_y > self.vert_line {
            return;
        }
        if self.window_x < 7 || self.window_x > 166 {
            return;
        }
        let screen_x = self.window_x - 7;
        let window_offset = (self.vert_line - self.window_y);

        let mut canvasoffs = screen_x as usize + (self.vert_line as usize) * SCREEN_WIDTH;

        let mut tile_x = 0;
        let tile_y = window_offset & 7;

        let mut line_offset = ((window_offset) >> 3) << 5;
        let mut tile = self.vram[(tilemap_addr_start + line_offset as u16) as usize] as u16;

        for _ in (screen_x as usize)..SCREEN_WIDTH {
            let pal_color = self.background_palette[self.bg_tiles[tile as usize][tile_y as usize][tile_x] as usize];

            if self.use_zero_as_window_solid() ||
                (pal_color != GBColor::White && !self.use_zero_as_window_solid()) {
                self.pixels[canvasoffs as usize] = pal_color;
            }

            canvasoffs += 1;
            tile_x += 1;
            if tile_x == 8 {
                tile_x = 0;
                line_offset += 1;
                tile = self.vram[(tilemap_addr_start as usize + line_offset as usize) as usize] as u16;
                if !self.tile_pattern_table_address() && tile < 128 {
                    tile += 256;
                }
            }
        }
    }

    fn render_window(&mut self) {

        // VRAM offset for the tile map
        let mut map_offs: u16 = 0x1800;
        if self.window_tile_table_address() {
            map_offs = 0x1C00
        }
        if self.vert_line < self.window_y {
            return;
        }
        if self.window_x < 7 || self.window_x > 166 {
            return;
        }

        let screen_x = (self.window_x - 7);
        let screen_y = ((self.vert_line - self.window_y));
        map_offs += (screen_y >> 3) as u16;

        // Where to render on the canvas
        let mut canvasoffs: u32 = ((screen_x as usize + (self.vert_line as usize * SCREEN_WIDTH))) as u32;

        // Which tile to start with in the map line
        let mut line_offset: u16 = 0;

        // Read tile index from the background map
        let mut tile = self.vram[(map_offs + line_offset) as usize] as u16;

        // If the tile data set in use is #1, the
        // indices are signed; calculate a real tile offset
        if self.tile_pattern_table_address() && tile < 128 {
            tile += 256;
        }

        let mut tile_x = ((screen_x) & 7) as usize;
        let tile_y = ((screen_y & 7) as usize);

        for _ in self.window_x..SCREEN_WIDTH as u8 {
            // Re-map the tile pixel through the palette
            let pal_color = self.background_palette[self.bg_tiles[tile as usize][tile_y][tile_x] as usize];

            // Plot the pixel to canvas…
            self.pixels[canvasoffs as usize] = pal_color;

            canvasoffs += 1;

            // When this tile ends, read another
            tile_x += 1;
            if tile_x == 8 {
                tile_x = 0;
                line_offset = (line_offset + 1) & 31; // tiles are 32x32
                tile = self.vram[(map_offs + line_offset) as usize] as u16;
                if !self.tile_pattern_table_address() && tile < 128 {
                    tile += 256;
                }
            }
        }
    }
    fn render_objects(&mut self) {
        let use_8x16 = self.use_8x16_sprites();
        let mut objects_drawn = HashMap::new();
        for obj in self.objects.iter() {
            if obj.x == 0 && obj.y == 0 {
                continue;
            }
            let screen_x = (obj.x as i32 - 8);
            let screen_y = (obj.y as i32 - 16);
            let sprite_pattern = (if use_8x16 { obj.pattern_num & 0xFE } else { obj.pattern_num }) as usize;


            for y in 0..(if use_8x16 { 16 } else { 8 }) {
                let mut drawn_any = false;
                let entry_key = screen_y + y;
                let entry = objects_drawn.entry(entry_key.clone()).or_insert(0);
                if entry >= &mut 10 {
                    continue;
                }
                for x in 0..8 {
                    if (screen_x + x) < 0 || (screen_y + y) < 0 {
                        continue;
                    }
                    drawn_any = true;
                    let palette = match obj.pal_num {
                        false => self.object_palette0,
                        true => self.object_palette1
                    };
                    let sprite_x = (if obj.x_flip { 7 - x } else { x }) as usize;
                    let sprite_y = (if obj.y_flip { 7 - y } else { y }) as usize;
                    let pal_color = palette
                        [self.bg_tiles
                        [sprite_pattern][sprite_y][sprite_x] as usize];

                    if pal_color == GBColor::White {
                        continue;
                    }
                    let pos = (screen_x + x) as usize + (screen_y + y) as usize * SCREEN_WIDTH;
                    if !obj.priority || (obj.priority && self.pixels[pos] == GBColor::White) {
                        self.pixels[pos] = pal_color;
                    }
                }

                if drawn_any {
                    *entry += 1;
                }
            }
        }
    }

    pub(crate) fn get_bg_tiles(&self) -> &[Tile16; 384] {
        &self.bg_tiles
    }
}
