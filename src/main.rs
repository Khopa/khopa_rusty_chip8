use crate::debug_utils::{print_registers, print_display, print_memory};
use crate::chip8::{load_program, step};
use std::borrow::{Borrow, BorrowMut};
use std::{time, thread};
use std::thread::sleep;

mod chip8;
mod chip8_display;
mod chip8_memory;
mod debug_utils;
mod chip8_instructions;

fn main() {

    let mut device = chip8::build_chip8();
    print_registers(&device);
    print_display(&device);
    load_program(device.borrow_mut(), "./resources/IBM");
    print_memory(&device);

    println!();

    //chip8_instructions::get_and_print_instruction_type(0x06E5);
    //chip8_instructions::get_and_print_instruction_type(0x00E0);
    //chip8_instructions::get_and_print_instruction_type(0x00EE);
    //chip8_instructions::get_and_print_instruction_type(0xB0E0);
    println!("PRG START");

    for i in 0..64 {
        step(device.borrow_mut());
        println!("CPU CYCLE ## {}", i);
        print_registers(&device);
        //sleep(time::Duration::from_millis(2));
    }

    print_display(&device);

}
