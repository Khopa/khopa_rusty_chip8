
pub const DISPLAY_WIDTH: usize = 64;
pub const DISPLAY_HEIGHT: usize = 32;

pub struct Chip8Display{
    pub(crate) display_data: [[u8; DISPLAY_WIDTH/8];DISPLAY_HEIGHT],
}

pub fn build_chip8_display() -> Chip8Display {
    let mut display = Chip8Display{
        display_data: [[0b00000000; DISPLAY_WIDTH/8];DISPLAY_HEIGHT]
    };
    return display;
}

pub fn xor_px_at(display: &mut Chip8Display, x:usize, y:usize) -> bool{
    let mut ry = y;
    let mut rx = x;

    if rx >= DISPLAY_WIDTH{ rx = DISPLAY_WIDTH-1; }
    if ry >= DISPLAY_HEIGHT{ ry = DISPLAY_HEIGHT-1; }

    let mut xored = false;
    if (display.display_data[ry][rx/8] & (0b1000000 >> (rx%8))) > 0 {
        xored = true;
    }else{
        xored = false;
    }

    display.display_data[ry][rx/8] ^= (0b1000000 >> (rx%8)) as u8;

    return xored;
}

/**
 * Default Chip 8 Sprites
 */
pub const SPRITE_0: [u8; 5] = [0xF0, 0x90, 0x90, 0x90, 0xF0];
pub const SPRITE_1: [u8; 5] = [0x20, 0x60, 0x20, 0x20, 0x70];
pub const SPRITE_2: [u8; 5] = [0xF0, 0x10, 0xF0, 0x80, 0xF0];
pub const SPRITE_3: [u8; 5] = [0xF0, 0x10, 0xF0, 0x10, 0xF0];
pub const SPRITE_4: [u8; 5] = [0x90, 0x90, 0xF0, 0x10, 0x10];
pub const SPRITE_5: [u8; 5] = [0xF0, 0x80, 0xF0, 0x10, 0xF0];
pub const SPRITE_6: [u8; 5] = [0xF0, 0x80, 0xF0, 0x90, 0xF0];
pub const SPRITE_7: [u8; 5] = [0xF0, 0x10, 0x20, 0x40, 0x40];
pub const SPRITE_8: [u8; 5] = [0xF0, 0x90, 0xF0, 0x90, 0xF0];
pub const SPRITE_9: [u8; 5] = [0xF0, 0x90, 0xF0, 0x10, 0xF0];
pub const SPRITE_A: [u8; 5] = [0xF0, 0x90, 0xF0, 0x90, 0x90];
pub const SPRITE_B: [u8; 5] = [0xE0, 0x90, 0xE0, 0x10, 0xE0];
pub const SPRITE_C: [u8; 5] = [0xF0, 0x80, 0x80, 0x80, 0xF0];
pub const SPRITE_D: [u8; 5] = [0xE0, 0x90, 0x90, 0x90, 0xE0];
pub const SPRITE_E: [u8; 5] = [0xF0, 0x80, 0xF0, 0x80, 0xF0];
pub const SPRITE_F: [u8; 5] = [0xF0, 0x80, 0xF0, 0x80, 0x80];

pub const DEFAULT_SPRITE:[[u8; 5]; 16] = [SPRITE_0, SPRITE_1,SPRITE_2,SPRITE_3,
                                          SPRITE_4,SPRITE_5,SPRITE_6,SPRITE_7,
                                          SPRITE_8,SPRITE_9,SPRITE_A,SPRITE_B,
                                          SPRITE_C,SPRITE_D,SPRITE_E,SPRITE_F];
