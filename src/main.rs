use crate::debug_utils::print_registers;

mod chip8;
mod chip8_display;
mod chip8_memory;
mod debug_utils;

fn main() {
    println!("Hello, world!");

    let chip8 = chip8::build_chip8();
    print_registers(chip8);

}
