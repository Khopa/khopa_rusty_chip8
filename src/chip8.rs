use std::fs::File;
use std::io::Read;

use crate::chip8_display;
use crate::chip8_memory;
use crate::chip8_instructions::{get_and_print_instruction_type, exec};
use std::ops::Shl;
use std::borrow::{BorrowMut, Borrow};
use crate::chip8_display::{SPRITE_0, DEFAULT_SPRITE};

pub const REGISTER_COUNT: usize = 16;
pub const STACK_SIZE: usize = 16;
pub const KEYBOARD_SIZE: usize = 16;
pub const CLOCK_SPEED: u16 = 540;

pub struct Chip8{

    // Memory
    pub(crate) memory: [u8; chip8_memory::END_MEM],

    // General Purpose Registers
    pub(crate) vn: [u8; REGISTER_COUNT],

    // Delay Registers
    pub(crate) dt: u8,

    // Sound register
    pub(crate) st: u8,

    // I Register
    pub(crate) i: u16,

    // Display
    pub(crate) display: chip8_display::Chip8Display,

    // Keyboard
    pub(crate) keyboard: [bool; KEYBOARD_SIZE],

    // Last pressed key
    pub(crate) key: usize,

    // Program counter
    pub(crate) pc: u16,

    // Stack pointer
    pub(crate) sp: u8,

    // Stack
    pub(crate) stack: [u16; STACK_SIZE],

    // Cycle
    pub(crate) cycle: u16,

    pub(crate) turbo: bool,

}

/**
 * Create a Chip 8 struct
 */
pub fn build_chip8() -> Chip8{
    let mut device = Chip8{
        memory: [0; chip8_memory::END_MEM],
        vn: [0; REGISTER_COUNT],
        dt: 0,
        st: 0,
        i: 0,
        display: chip8_display::build_chip8_display(),
        keyboard: [false; KEYBOARD_SIZE],
        key: KEYBOARD_SIZE + 1,
        pc: chip8_memory::START_PRG as u16,
        sp: 0,
        stack: [0; STACK_SIZE],
        cycle: 0,
        turbo: false
    };
    load_default_sprites(device.borrow_mut());
    return device;
}

/**
 * Load all the default sprite in the device memory
 */
fn load_default_sprites(device: &mut Chip8){
    let mut address:usize = 0x00;
    for sprite in 0..DEFAULT_SPRITE.len(){
        load_sprite_at(device.borrow_mut(), address, DEFAULT_SPRITE[sprite]);
        address = address + 5;
    }
}

/**
 * Put sprite data in memory
 */
fn load_sprite_at(device: &mut Chip8, address: usize, sprite: [u8; 5]){
    for i in 0..5{
        device.memory[address + i] = sprite[i];
    }
}

/**
 * Step
 */
pub fn step(device: &mut Chip8){
    let instruction:u16 = (device.memory[(device.pc+ 1) as usize] as u16) + (device.memory[device.pc as usize] as u16).shl(8);
    exec(instruction, device);
    if device.cycle >= CLOCK_SPEED/60{
        device.cycle = 0;
        if device.dt > 0{
            device.dt -= 1;
        }
        if device.st > 0{
            device.st -= 1;
        }
    }
    device.cycle += 1;
}

/**
 * Load a Chip 8 program in memory
 */
pub fn load_program(device: &mut Chip8, path: &str) -> bool{

    let file = File::open(path);
    let mut file = match file {
        Ok(file) => file,
        Err(e) => {
            panic!("Error : not a valid ROM file");
        }
    };

    // Do read the file
    let mut mem: usize = chip8_memory::START_PRG;
    for byte in file.bytes() {
        let b = byte.unwrap();
        device.memory[mem] = b;
        mem = mem + 1;
    }

    return true;
}
