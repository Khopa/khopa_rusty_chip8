extern crate rand;
use crate::chip8::Chip8;
use crate::chip8_display;
use crate::chip8_display::{DISPLAY_HEIGHT, DISPLAY_WIDTH};

use rand::Rng;
use std::ops::SubAssign;

#[derive(Debug)]
#[derive(PartialEq, Eq)]
pub enum CH8_INSTRUCTION {
    SYS,
    CLS,
    RET,
    JP,
    CALL,
    SE,
    SNE,
    SEVxVy,
    LDVxbyte,
    ADDVxbyte,
    LDVxVy,
    OR,
    AND,
    XOR,
    ADDVxVy,
    SUBVxVy,
    SHR,
    SUBN,
    SHL,
    SNEVxVy,
    LDIaddr,
    JPV0addr,
    RND,
    DRW,
    SKPVx,
    SKNPVx,
    LDVxDT,
    LDVxK,
    LDDTVx,
    LDSTVx,
    ADDIVx,
    LDFVx,
    LDBVx,
    LDIVx,
    LDVxI,
    NOOP,
}

pub fn get_and_print_instruction_type(ins: u16) -> CH8_INSTRUCTION {
    let a = get_instruction_type(ins);
    println!("{:#016b} {:#04x?} | {:.16?}", ins, ins, a);
    return a;
}

pub fn get_instruction_type(ins: u16) -> CH8_INSTRUCTION {
    if ins == 0x00E0 {
        return CH8_INSTRUCTION::CLS;
    } else if ins == 0x00EE {
        return CH8_INSTRUCTION::RET;
    } else if ins & 0xF000 == 0x0000 {
        return CH8_INSTRUCTION::SYS;
    } else if ins & 0xF000 == 0x1000 {
        return CH8_INSTRUCTION::JP;
    } else if ins & 0xF000 == 0x2000 {
        return CH8_INSTRUCTION::CALL;
    } else if ins & 0xF000 == 0x3000 {
        return CH8_INSTRUCTION::SE;
    } else if ins & 0xF000 == 0x4000 {
        return CH8_INSTRUCTION::SNE;
    } else if ins & 0xF000 == 0x5000 {
        return CH8_INSTRUCTION::SNE;
    } else if ins & 0xF000 == 0x6000 {
        return CH8_INSTRUCTION::SEVxVy;
    } else if ins & 0xF000 == 0x7000 {
        return CH8_INSTRUCTION::ADDVxbyte;
    } else if ins & 0xF00F == 0x8001 {
        return CH8_INSTRUCTION::OR;
    } else if ins & 0xF00F == 0x8002 {
        return CH8_INSTRUCTION::AND;
    } else if ins & 0xF00F == 0x8003 {
        return CH8_INSTRUCTION::XOR;
    } else if ins & 0xF00F == 0x8004 {
        return CH8_INSTRUCTION::ADDVxVy;
    } else if ins & 0xF00F == 0x8005 {
        return CH8_INSTRUCTION::SUBVxVy;
    } else if ins & 0xF00F == 0x8006 {
        return CH8_INSTRUCTION::SHR;
    } else if ins & 0xF00F == 0x8007 {
        return CH8_INSTRUCTION::SUBN;
    } else if ins & 0xF00F == 0x800E {
        return CH8_INSTRUCTION::SHL;
    } else if ins & 0xF000 == 0x9000 {
        return CH8_INSTRUCTION::SNEVxVy;
    } else if ins & 0xF000 == 0xA000 {
        return CH8_INSTRUCTION::LDIaddr;
    } else if ins & 0xF000 == 0xB000 {
        return CH8_INSTRUCTION::JPV0addr;
    } else if ins & 0xF000 == 0xC000 {
        return CH8_INSTRUCTION::RND;
    } else if ins & 0xF000 == 0xD000 {
        return CH8_INSTRUCTION::DRW;
    } else if ins & 0xF0FF == 0xE09E {
        return CH8_INSTRUCTION::SKPVx;
    } else if ins & 0xF0FF == 0xE0A1 {
        return CH8_INSTRUCTION::SKNPVx;
    } else if ins & 0xF0FF == 0xF007 {
        return CH8_INSTRUCTION::LDVxDT;
    } else if ins & 0xF0FF == 0xF00A {
        return CH8_INSTRUCTION::LDVxK;
    } else if ins & 0xF0FF == 0xF015 {
        return CH8_INSTRUCTION::LDDTVx;
    } else if ins & 0xF0FF == 0xF018 {
        return CH8_INSTRUCTION::LDSTVx;
    } else if ins & 0xF0FF == 0xF01E {
        return CH8_INSTRUCTION::ADDIVx;
    } else if ins & 0xF0FF == 0xF029 {
        return CH8_INSTRUCTION::LDFVx;
    } else if ins & 0xF0FF == 0xF033 {
        return CH8_INSTRUCTION::LDBVx;
    } else if ins & 0xF0FF == 0xF055 {
        return CH8_INSTRUCTION::LDIVx;
    } else if ins & 0xF0FF == 0xF055 {
        return CH8_INSTRUCTION::LDVxI;
    }
    return CH8_INSTRUCTION::NOOP;
}

pub fn exec(ins: u16, device: &mut Chip8){
    let itype = get_instruction_type(ins);
    println!("{:#04x?} | {:#016b} {:#04x?} | {:.16?}", device.pc, ins, ins, itype);
    match itype {
        CH8_INSTRUCTION::SYS => { sys(device, ins); },
        CH8_INSTRUCTION::CLS => { cls(device, ins); },
        CH8_INSTRUCTION::RET => { ret(device, ins); },
        CH8_INSTRUCTION::JP => { jp(device, ins); },
        CH8_INSTRUCTION::CALL => { call(device, ins); },
        CH8_INSTRUCTION::SE => { se(device, ins); },
        CH8_INSTRUCTION::SNE => { sne(device, ins); },
        CH8_INSTRUCTION::SEVxVy => { sevxvy(device, ins); },
        CH8_INSTRUCTION::LDVxbyte => { ldvxb(device, ins); },
        CH8_INSTRUCTION::ADDVxbyte => { addvxb(device, ins); },
        CH8_INSTRUCTION::LDVxVy => { ldvxvy(device, ins); },
        CH8_INSTRUCTION::OR => { or(device, ins); },
        CH8_INSTRUCTION::AND => { and(device, ins); },
        CH8_INSTRUCTION::XOR => { xor(device, ins); },
        CH8_INSTRUCTION::ADDVxVy => { addvxvy(device, ins); },
        CH8_INSTRUCTION::SUBVxVy => { subvxvy(device, ins); },
        CH8_INSTRUCTION::SHR => { shr(device, ins); },
        CH8_INSTRUCTION::SUBN => { subnvxvy(device, ins);},
        CH8_INSTRUCTION::SHL => { shl(device, ins); },
        CH8_INSTRUCTION::SNEVxVy => { snevxvy(device, ins); },
        CH8_INSTRUCTION::LDIaddr => {},
        CH8_INSTRUCTION::JPV0addr => {},
        CH8_INSTRUCTION::RND => { rnd(device, ins); },
        CH8_INSTRUCTION::DRW => {},
        CH8_INSTRUCTION::SKPVx => {},
        CH8_INSTRUCTION::SKNPVx => {},
        CH8_INSTRUCTION::LDVxDT => {},
        CH8_INSTRUCTION::LDVxK => {},
        CH8_INSTRUCTION::LDDTVx => {},
        CH8_INSTRUCTION::LDSTVx => {},
        CH8_INSTRUCTION::ADDIVx => {},
        CH8_INSTRUCTION::LDFVx => {},
        CH8_INSTRUCTION::LDBVx => {},
        CH8_INSTRUCTION::LDIVx => {},
        CH8_INSTRUCTION::LDVxI => {},
        CH8_INSTRUCTION::NOOP => {},
    }
    return;
}

/**
* 0nnn - SYS addr
* Jump to a machine code routine at nnn.
*This instruction is only used on the old computers on which Chip-8 was originally implemented. It is ignored by modern interpreters.
**/
fn sys(device: &mut Chip8, ins: u16) {
    // Do nothing
}

/*
00E0 - CLS
Clear the display.
*/
fn cls(device: &mut Chip8, ins: u16) {
    for i in 0..DISPLAY_HEIGHT {
        for j in 0..DISPLAY_WIDTH / 8 {
            device.display.display_data[i][j] = 0x00;
        }
    }
}

/*
00EE - RET
Return from a subroutine.

The interpreter sets the program counter to the address at the top of the stack, then subtracts 1 from the stack pointer.
*/
fn ret(device: &mut Chip8, ins: u16) {
    let nnn:u16 = ins & 0x0FFF;
    device.pc = device.stack[device.sp as usize];
    device.sp = device.sp - 1;
}

/*
1nnn - JP addr
Jump to location nnn.

The interpreter sets the program counter to nnn.
*/
fn jp(device: &mut Chip8, ins: u16) {
    let nnn:u16 = ins & 0x0FFF;
    println!("{:#4x?}", nnn);
    device.pc = nnn;
}

/*
2nnn - CALL addr
Call subroutine at nnn.

The interpreter increments the stack pointer, then puts the current PC on the top of the stack. The PC is then set to nnn.
*/
fn call(device: &mut Chip8, ins: u16) {
    let nnn:u16 = ins & 0x0FFF;
    device.sp = device.sp + 1;
    device.stack[device.sp as usize] = device.pc;
    device.pc = nnn;
}

/*
3xkk - SE Vx, byte
Skip next instruction if Vx = kk.

The interpreter compares register Vx to kk, and if they are equal, increments the program counter by 2.
*/
fn se(device: &mut Chip8, ins: u16) {
    let register:usize = (ins & 0x0F00 >> 8) as usize;
    let byte:u8 = (ins & 0x00FF) as u8;
    if device.vn[register] == byte {
        device.pc += 2;
    }
}

/*
4xkk - SNE Vx, byte
Skip next instruction if Vx != kk.

The interpreter compares register Vx to kk, and if they are not equal, increments the program counter by 2.
*/
fn sne(device: &mut Chip8, ins: u16) {
    let register:usize = (ins & 0x0F00 >> 8) as usize;
    let byte:u8 = (ins & 0x00FF) as u8;
    if device.vn[register] != byte {
        device.pc += 2;
    }
}

/*
5xy0 - SE Vx, Vy
Skip next instruction if Vx = Vy.

The interpreter compares register Vx to register Vy, and if they are equal, increments the program counter by 2.
*/
fn sevxvy(device: &mut Chip8, ins: u16) {
    let x:usize = (ins & 0x0F00 >> 8) as usize;
    let y:usize = (ins & 0x00F0 >> 4) as usize;
    if device.vn[x] == device.vn[y] {
        device.pc += 2;
    }
}

/*
6xkk - LD Vx, byte
Set Vx = kk.

The interpreter puts the value kk into register Vx.
*/
fn ldvxb(device: &mut Chip8, ins: u16) {
    let register:usize = (ins & 0x0F00 >> 8) as usize;
    let byte:u8  = (ins & 0x00FF) as u8;
    device.vn[register] = byte;
}

/*
7xkk - ADD Vx, byte
Set Vx = Vx + kk.

Adds the value kk to the value of register Vx, then stores the result in Vx.
*/
fn addvxb(device: &mut Chip8, ins: u16) {
    let register:usize = (ins & 0x0F00 >> 8) as usize;
    let byte:u8 = (ins & 0x00FF) as u8;
    device.vn[register] = device.vn[register] + byte;
}


/*
8xy0 - LD Vx, Vy
Set Vx = Vy.

Stores the value of register Vy in register Vx.
*/
fn ldvxvy(device: &mut Chip8, ins: u16) {
    let x:usize = (ins & 0x0F00 >> 8) as usize;
    let y:usize = (ins & 0x00F0 >> 4) as usize;
    device.vn[x] = device.vn[y];
}


/*
8xy1 - OR Vx, Vy
Set Vx = Vx OR Vy.

Performs a bitwise OR on the values of Vx and Vy, then stores the result in Vx.
*/
fn or(device: &mut Chip8, ins: u16) {
    let x:usize = (ins & 0x0F00 >> 8) as usize;
    let y:usize = (ins & 0x00F0 >> 4) as usize;
    device.vn[x] = device.vn[x] | device.vn[y];
}


/*
8xy2 - AND Vx, Vy
Set Vx = Vx AND Vy.

Performs a bitwise AND on the values of Vx and Vy, then stores the result in Vx.

*/
fn and(device: &mut Chip8, ins: u16) {
    let x:usize = (ins & 0x0F00 >> 8) as usize;
    let y:usize = (ins & 0x00F0 >> 4) as usize;
    device.vn[x] = device.vn[x] & device.vn[y];
}

/*
8xy3 - XOR Vx, Vy
Set Vx = Vx XOR Vy.

Performs a bitwise exclusive OR on the values of Vx and Vy, then stores the result in Vx.

*/
fn xor(device: &mut Chip8, ins: u16) {
    let x:usize = (ins & 0x0F00 >> 8) as usize;
    let y:usize = (ins & 0x00F0 >> 4) as usize;
    device.vn[x] = device.vn[x] ^ device.vn[y];
}

/*
8xy4 - ADD Vx, Vy
Set Vx = Vx + Vy, set VF = carry.

The values of Vx and Vy are added together. If the result is greater than 8 bits (i.e., > 255,)
VF is set to 1, otherwise 0. Only the lowest 8 bits of the result are kept, and stored in Vx.
*/
fn addvxvy(device: &mut Chip8, ins: u16) {
    let x:usize = (ins & 0x0F00 >> 8) as usize;
    let y:usize = (ins & 0x00F0 >> 4) as usize;
    let result:u16 = (device.vn[x] as u16) + (device.vn[y] as u16);
    if result > 0xFF { device.vf = 1; } else { device.vf = 0; };
    device.vn[x] = (result & 0x00FF) as u8;
}

/*
8xy5 - SUB Vx, Vy
Set Vx = Vx - Vy, set VF = NOT borrow.

If Vx > Vy, then VF is set to 1, otherwise 0. Then Vy is subtracted from Vx, and the results stored in Vx.
*/
fn subvxvy(device: &mut Chip8, ins: u16) {
    let x:usize = (ins & 0x0F00 >> 8) as usize;
    let y:usize = (ins & 0x00F0 >> 4) as usize;

    if device.vn[x] > device.vn[y] { device.vf = 1;} else { device.vf = 0; }
    device.vn[x] = device.vn[x] - device.vn[y]
}

/*
8xy6 - SHR Vx {, Vy}
Set Vx = Vy SHR 1.

https://github.com/mattmikolay/chip-8/wiki/CHIP%E2%80%908-Instruction-Set#notes
Store the value of register VY shifted right one bit in register VX¹
Set register VF to the least significant bit prior to the shift
VY is unchanged

*/
fn shr(device: &mut Chip8, ins: u16) {
    let x:usize = (ins & 0x0F00 >> 8) as usize;
    let y:usize = (ins & 0x00F0 >> 4) as usize;

    device.vf = device.vn[y]&0x01;
    device.vn[x] = device.vn[y].wrapping_shr(1);
}

/*
8xy7 - SUBN Vx, Vy
Set Vx = Vy - Vx, set VF = NOT borrow.

If Vy > Vx, then VF is set to 1, otherwise 0. Then Vx is subtracted from Vy, and the results stored in Vx.

Set register VX to the value of VY minus VX
Set VF to 00 if a borrow occurs
Set VF to 01 if a borrow does not occur

*/
fn subnvxvy(device: &mut Chip8, ins: u16) {
    let x: usize = (ins & 0x0F00 >> 8) as usize;
    let y: usize = (ins & 0x00F0 >> 4) as usize;

    if device.vn[y] > device.vn[x] {
        device.vf = 1;
        device.vn[x] = device.vn[y] - device.vn[x];
    } else {
        device.vf = 0;
        device.vn[x] = 0;
    }

}

/*
8xyE - SHL Vx {, Vy}
Set Vx = Vx SHL 1.

If the most-significant bit of Vx is 1, then VF is set to 1, otherwise to 0. Then Vx is multiplied by 2.

https://github.com/mattmikolay/chip-8/wiki/CHIP%E2%80%908-Instruction-Set#notes
Store the value of register VY shifted left one bit in register VX¹
Set register VF to the most significant bit prior to the shift
VY is unchanged

*/
fn shl(device: &mut Chip8, ins: u16) {
    let x:usize = (ins & 0x0F00 >> 8) as usize;
    let y:usize = (ins & 0x00F0 >> 4) as usize;

    device.vf = device.vn[y]&0x80 >> 7;
    device.vn[x] = device.vn[y].wrapping_shl(1);
}

/*
9xy0 - SNE Vx, Vy
Skip next instruction if Vx != Vy.

The values of Vx and Vy are compared, and if they are not equal, the program counter is increased by 2.
*/
fn snevxvy(device: &mut Chip8, ins: u16) {
    let x:usize = (ins & 0x0F00 >> 8) as usize;
    let y:usize = (ins & 0x00F0 >> 4) as usize;

    if device.vn[x] != device.vn[y] {
        device.pc = device.pc + 2;
    }
}

/*
Annn - LD I, addr
Set I = nnn.

The value of register I is set to nnn.
*/

/*
Bnnn - JP V0, addr
Jump to location nnn + V0.

The program counter is set to nnn plus the value of V0.
*/

/*
Cxkk - RND Vx, byte
Set Vx = random byte AND kk.

The interpreter generates a random number from 0 to 255, which is then ANDed with the value kk. The results are stored in Vx. See instruction 8xy2 for more information on AND.
*/
fn rnd(device: &mut Chip8, ins: u16) {
    let register:usize = (ins & 0x0F00 >> 8) as usize;
    let byte = (ins & 0x00FF) as u8;
    let rand = rand::thread_rng().gen_range(0, 255);
    device.vn[register] = rand & byte;
    println!("{}", rand);
}


/*
Dxyn - DRW Vx, Vy, nibble
Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision.

The interpreter reads n bytes from memory, starting at the address stored in I. These bytes are then displayed as sprites on screen at coordinates (Vx, Vy). Sprites are XORed onto the existing screen. If this causes any pixels to be erased, VF is set to 1, otherwise it is set to 0. If the sprite is positioned so part of it is outside the coordinates of the display, it wraps around to the opposite side of the screen. See instruction 8xy3 for more information on XOR, and section 2.4, Display, for more information on the Chip-8 screen and sprites.
*/

/*
Ex9E - SKP Vx
Skip next instruction if key with the value of Vx is pressed.

Checks the keyboard, and if the key corresponding to the value of Vx is currently in the down position, PC is increased by 2.
*/

/*
ExA1 - SKNP Vx
Skip next instruction if key with the value of Vx is not pressed.

Checks the keyboard, and if the key corresponding to the value of Vx is currently in the up position, PC is increased by 2.
*/

/*
Fx07 - LD Vx, DT
Set Vx = delay timer value.

The value of DT is placed into Vx.
*/

/*
Fx0A - LD Vx, K
Wait for a key press, store the value of the key in Vx.

All execution stops until a key is pressed, then the value of that key is stored in Vx.
*/

/*
Fx15 - LD DT, Vx
Set delay timer = Vx.

DT is set equal to the value of Vx.
*/

/*
Fx18 - LD ST, Vx
Set sound timer = Vx.

ST is set equal to the value of Vx.
*/

/*
Fx1E - ADD I, Vx
Set I = I + Vx.

The values of I and Vx are added, and the results are stored in I.
*/

/*
Fx29 - LD F, Vx
Set I = location of sprite for digit Vx.

The value of I is set to the location for the hexadecimal sprite corresponding to the value of Vx. See section 2.4, Display, for more information on the Chip-8 hexadecimal font.
*/

/*
Fx33 - LD B, Vx
Store BCD representation of Vx in memory locations I, I+1, and I+2.

The interpreter takes the decimal value of Vx, and places the hundreds digit in memory at location in I, the tens digit at location I+1, and the ones digit at location I+2.
*/

/*
Fx55 - LD [I], Vx
Store registers V0 through Vx in memory starting at location I.

The interpreter copies the values of registers V0 through Vx into memory, starting at the address in I.
*/

/*
Fx65 - LD Vx, [I]
Read registers V0 through Vx from memory starting at location I.

The interpreter reads values from memory starting at location I into registers V0 through Vx.
*/