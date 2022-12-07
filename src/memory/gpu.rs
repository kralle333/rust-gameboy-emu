use sdl2::{pixels::Color, rect::Rect, render::Canvas, video::Window};

use super::MemoryType;
use crate::video::{self, SCREEN_WIDTH};

type ColorByte = u8;
type Tile16 = [[ColorByte; 8]; 8];

fn make_tile16() -> Tile16 {
    [[0; 8]; 8]
}

#[derive(Clone, Copy)]
struct ObjData {
    x: i32,
    y: i32,
    pattern_num: i32,
    priority: i32,
    y_flip: i32,
    x_flip: i32,
    pal_num: i32,
}

impl ObjData {
    fn new() -> Self {
        Self {
            x: 0,
            y: 0,
            pattern_num: 0,
            priority: 0,
            y_flip: 0,
            x_flip: 0,
            pal_num: 0,
        }
    }
}

pub struct Gpu {
    vram: [u8; 0x2000],
    objects: [ObjData; 40],
    oam: [u8; 0xA0],
    bg_tiles: [Tile16; 384],
    //Video registers
    lcdc: u8,                           //FF40
    lcdc_stat: u8,                      //FF41
    scroll_x: u8,                       //FF42
    scroll_y: u8,                       //FF43
    vert_line: u8,                      //FF44
    vert_line_cp: u8,                   //FF45
    window_y: u8,                       //FF4A
    window_x: u8,                       //FF4B
    bg_palette: u8,                     //FF47,
    obj_palette0: u8,                   //FF48
    obj_palette1: u8,                   //FF49
    background_palette: [ColorByte; 4], //FF47
    object_palette0: [ColorByte; 4],    //FF48
    object_palette1: [ColorByte; 4],    //FF49
    pixels: [ColorByte; (video::SCREEN_WIDTH * video::SCREEN_HEIGHT) as usize],
}

impl MemoryType for Gpu {
    fn read_byte(&self, addr: u16) -> u8 {
        match addr {
            0x8000..=0x9fff => self.vram[(addr & 0x1fff) as usize],
            0xfe00..=0xfea0 => self.oam[(addr & 160) as usize],
            0xff40..=0xff49 => match addr & 0x00FF {
                0x40 => self.lcdc,
                0x41 => self.lcdc_stat,
                0x42 => self.scroll_y,
                0x43 => self.scroll_x,
                0x44 => self.vert_line,
                0x45 => self.vert_line_cp,
                0xFA => self.window_x,
                0xFB => self.window_y,
                0x47 => self.bg_palette,
                0x48 => self.obj_palette0,
                0x49 => self.obj_palette1,
                _ => panic!("video flags"),
            },
            _ => panic!("video"),
        }
    }

    fn write_byte(&mut self, addr: u16, val: u8) {
        match addr {
            0x8000..=0x9fff => self.vram[(addr & 0x1fff) as usize] = val,
            0xfe00..=0xfea0 => self.oam[(addr & 160) as usize] = val,
            0xff40..=0xff49 => match addr & 0x00FF {
                0x40 => self.lcdc  = val,
                0x41 => self.lcdc_stat  = val,
                0x42 => self.scroll_y  = val,
                0x43 => self.scroll_x  = val,
                0x44 => self.vert_line  = val,
                0x45 => self.vert_line_cp  = val,
                0xFA => self.window_x  = val,
                0xFB => self.window_y  = val,
                0x47 => self.bg_palette  = val,
                0x48 => self.obj_palette0  = val,
                0x49 => self.obj_palette1  = val,
                _ => panic!("video flags"),
            },
            _ => panic!("video"),
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
            pixels: [0; (video::SCREEN_WIDTH * video::SCREEN_HEIGHT) as usize],
            background_palette: [0; 4],
            object_palette0: [0; 4],
            object_palette1: [0; 4],
        }
    }
    pub fn draw(&mut self, canvas: &mut Canvas<Window>) {
        for x in 0..video::SCREEN_WIDTH-1 {
            for y in 0..video::SCREEN_HEIGHT-1 {
                let color = match self.pixels[x + video::SCREEN_WIDTH * y] {
                    0 => Color::RGBA(0x9C, 0xBD, 0x0F, 0xFF),
                    1 => Color::RGBA(0x8C, 0xAD, 0x0F, 0xFF),
                    2 => Color::RGBA(0x30, 0x62, 0x30, 0xFF),
                    3 => Color::RGBA(0x0F, 0x38, 0x0F, 0xFF),
                    _ => panic!("lol"),
                };

                canvas.set_draw_color(color);
                let x = x * video::PIXEL_SIZE;
                let y = y * video::PIXEL_SIZE;
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
        }
    }

    pub fn render_screen(&mut self) {
        //Display BG and window?
        if self.lcdc & 1 != 1 {
            return;
        }

        // VRAM offset for the tile map
        let mut map_offs: u32 = 0x1800;
        if self.lcdc & 0x8 == 0x8 {
            map_offs = 0x1C00
        }

        // Which line of tiles to use in the map
        map_offs += ((self.vert_line as u32 + self.scroll_y as u32) & 255) >> 3;

        // Which tile to start with in the map line
        let mut lineoffs: u32 = (self.scroll_x >> 3) as u32;

        // Which line of pixels to use in the tiles
        let y = ((self.vert_line + self.scroll_y) & 7) as usize;
        // Where in the tileline to start
        let mut x = (self.scroll_x & 7) as usize;

        // Where to render on the canvas
        let mut canvasoffs: u32 = (self.vert_line as usize * video::SCREEN_WIDTH) as u32;

        // Read tile index from the background map
        let mut tile: u32 = self.vram[(map_offs + lineoffs) as usize] as u32;

        // If the tile data set in use is #1, the
        // indices are signed; calculate a real tile offset
        let lcdl_4_set = self.lcdc & 0x10 == 0x10;
        if lcdl_4_set && tile < 128 {
            tile = tile.wrapping_add(0xff);
        }
        (0..SCREEN_WIDTH).for_each(|_i: usize| {
            // Re-map the tile pixel through the palette
            let pal_color = self.background_palette[self.bg_tiles[tile as usize][y][x] as usize];

            // Plot the pixel to canvas
            self.pixels[map_offs as usize] = pal_color;
            canvasoffs += 1;

            // When this tile ends, read another
            x += 1;
            if x == 8 {
                x = 0;
                lineoffs = (lineoffs + 1) & 31;
                tile = self.vram[(map_offs + lineoffs) as usize] as u32;
                if lcdl_4_set && tile < 128 {
                    tile = tile.wrapping_add(256);
                }
            }
        });
    }
}
