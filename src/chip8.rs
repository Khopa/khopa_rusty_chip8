use crate::chip8_display;

pub const REGISTER_COUNT: usize = 8;

pub struct Chip8{

    // General Purpose Registers
    pub(crate) v_registers: [u8; REGISTER_COUNT],

    // Delay Registers
    pub(crate) d_register: u8,

    // Sound register
    pub(crate) s_register: u8,

    // Display
    pub(crate) display: chip8_display::Chip8Display

}

pub fn build_chip8() -> Chip8{
    return Chip8{
        v_registers: [0,0,0,0,0,0,0,0],
        d_register: 0,
        s_register: 0,
        display: chip8_display::build_chip8_display()
    }
}
