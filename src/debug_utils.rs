use crate::chip8;

pub fn print_registers(device: chip8::Chip8){
    println!("----------------------------");
    println!("CHIP 8 Registers : ");
    println!("----------------------------");
    for n in 0..chip8::REGISTER_COUNT{
        println!("{} -> 0x{:02x?} | {:08b}", n, device.v_registers[n], device.v_registers[n]);
    }
    println!("----------------------------");
}