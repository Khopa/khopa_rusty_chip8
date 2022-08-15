use crate::chip8;
use crate::chip8_display;
use crate::chip8_display::Chip8Display;
use crate::chip8_memory;
use crate::chip8_memory::END_MEM;
use std::ops::Sub;
use crate::chip8::KEYBOARD_SIZE;

/**
 * Display the registers in console
 */
pub fn print_registers(device: &chip8::Chip8) {
    for n in 0..chip8::REGISTER_COUNT {
        print!("{:02?}|", device.vn[n]);
    }
    print!("| ");
}

/**
 * Display the Chip 8 screen in the console
 */
pub fn print_display(device: &chip8::Chip8) {
    for i in 0..chip8_display::DISPLAY_HEIGHT {
        print!("|");
        for j in 0..chip8_display::DISPLAY_WIDTH / 8 {
            for b in 0..8 {
                if device.display.display_data[i][j] & (0b10000000 >> b) > 0 {
                    print!("{}", "⬛");
                } else {
                    print!("{}", "⬜");
                }
            }
        }
        println!("|");
    }
}

/**
 * Display the Chip 8 screen in the console
 */
pub fn print_keyboard(device: &chip8::Chip8) {
    for i in 0..KEYBOARD_SIZE {
        if device.keyboard[i]{
            print!("{}", "⬛");
        } else {
            print!("{}", "⬜");
        }
    }
    println!("|")
}

/**
 * Display Memory
 *
 * Print the memory in this format :
 *
 * . . . . . . . .
 * . . . . . . . .
 * . . . . . . . .
 * . . . . . . . .
 * . . . . . . . .
 * . + + + + + + +
 *
 * . . . . . . . .
 * . . . . . + + +
 *
 * Each point represent 64 bytes of Data
 * When the memory is blank (filled with 0, we print ., else +)
 */
pub fn print_memory(device: &chip8::Chip8) {

    let mut position = END_MEM-1;
    for b in 0..(chip8_memory::END_MEM/64 + 1) {
        if b % 8 == 0 { println!() };
        let mut not_blank: bool = false;
        for i in 0..8 {
            if device.memory[position - i] > 0 {
                not_blank = true;
                break;
            };
            //println!("{}", position - i)
        }
        if not_blank { print!("+"); } else { print!("."); }
        if position > 64 {
            position = position - 64;
        }
    }

}


