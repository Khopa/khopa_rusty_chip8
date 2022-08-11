use crate::chip8::Chip8;
use fermium::prelude::*;

pub fn on_keyboard_event(device: &mut Chip8, keycode: SDL_Scancode, down: bool) {
    match keycode {
        SDL_SCANCODE_0 | SDL_SCANCODE_KP_0 => {
            device.keyboard[0x0] = down;
        }
        SDL_SCANCODE_1 | SDL_SCANCODE_KP_1 => {
            device.keyboard[0x1] = down;
        }
        SDL_SCANCODE_2 | SDL_SCANCODE_KP_2 => {
            device.keyboard[0x2] = down;
        }
        SDL_SCANCODE_3 | SDL_SCANCODE_KP_3 => {
            device.keyboard[0x3] = down;
        }
        SDL_SCANCODE_4 | SDL_SCANCODE_KP_4 => {
            device.keyboard[0x4] = down;
        }
        SDL_SCANCODE_5 | SDL_SCANCODE_KP_5 => {
            device.keyboard[0x5] = down;
        }
        SDL_SCANCODE_6 | SDL_SCANCODE_KP_6 => {
            device.keyboard[0x6] = down;
        }
        SDL_SCANCODE_7 | SDL_SCANCODE_KP_7 => {
            device.keyboard[0x7] = down;
        }
        SDL_SCANCODE_8 | SDL_SCANCODE_KP_8 => {
            device.keyboard[0x8] = down;
        }
        SDL_SCANCODE_9 | SDL_SCANCODE_KP_9 => {
            device.keyboard[0x9] = down;
        }
        SDL_SCANCODE_9 => {
            device.keyboard[0x9] = down;
        }
        SDL_SCANCODE_A => {
            device.keyboard[0xA] = down;
        }
        SDL_SCANCODE_B => {
            device.keyboard[0xB] = down;
        }
        SDL_SCANCODE_C => {
            device.keyboard[0xC] = down;
        }
        SDL_SCANCODE_D => {
            device.keyboard[0xD] = down;
        }
        SDL_SCANCODE_E => {
            device.keyboard[0xE] = down;
        }
        SDL_SCANCODE_F => {
            device.keyboard[0xF] = down;
        }
        _ => ()
    }
}
