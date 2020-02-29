use std::fs::File;
use std::io::Read;

use crate::chip8_display;
use crate::chip8_memory;

pub const REGISTER_COUNT: usize = 16;

pub struct Chip8{

    // Memory
    pub(crate) memory: [u8; chip8_memory::END_MEM],

    // General Purpose Registers
    pub(crate) v_registers: [u8; REGISTER_COUNT],

    // Delay Registers
    pub(crate) d_register: u8,

    // Sound register
    pub(crate) s_register: u8,

    // Display
    pub(crate) display: chip8_display::Chip8Display,

}

/**
 * Create a Chip 8 struct
 */
pub fn build_chip8() -> Chip8{
    return Chip8{
        memory: [0; chip8_memory::END_MEM],
        v_registers: [0; REGISTER_COUNT],
        d_register: 0,
        s_register: 0,
        display: chip8_display::build_chip8_display()
    }
}

/**
 * Load a Chip 8 program in memory
 */
pub fn load_program(device: &mut Chip8, path: &str) -> bool{

    let file = File::open(path);
    let mut file = match file {
        Ok(file) => file,
        Err(e) => {
            return false;
        }
    };

    // Do read the file
    let mut mem: usize = chip8_memory::START_PRG;
    for byte in file.bytes() {
        let b = byte.unwrap();
        print!("{}", b);
        device.memory[mem] = b;
        mem = mem + 1;
    }

    return true;
}
