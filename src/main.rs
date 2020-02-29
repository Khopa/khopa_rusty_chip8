use crate::debug_utils::{print_registers, print_display, print_memory};
use crate::chip8::load_program;
use std::borrow::{Borrow, BorrowMut};

mod chip8;
mod chip8_display;
mod chip8_memory;
mod debug_utils;

fn main() {
    println!("Hello, world!");

    let mut device = chip8::build_chip8();
    print_registers(&device);
    print_display(&device);
    load_program(device.borrow_mut(), "./resources/test_opcode.ch8");
    print_memory(&device);
}
