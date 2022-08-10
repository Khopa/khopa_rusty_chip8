#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(clippy::single_match)]

use crate::debug_utils::{print_registers, print_display, print_memory};
use crate::chip8::{load_program, step};
use std::borrow::{Borrow, BorrowMut};
use std::{time, thread};
use std::thread::sleep;
use std::convert::TryInto;

mod chip8;
mod chip8_display;
mod chip8_memory;
mod debug_utils;
mod chip8_instructions;

use fermium::{
    prelude::*,
    error::*, events::*, video::*, *,
};
use fermium::renderer::SDL_RenderDrawRect;
use fermium::prelude::{SDL_Rect, SDL_RenderDrawRectF};


unsafe fn render_chip8_display(renderer: *mut SDL_Renderer, device: &chip8::Chip8) {
    SDL_SetRenderDrawColor(renderer, 0, 255, 0, 255);
    for i in 0..chip8_display::DISPLAY_HEIGHT {
        for j in 0..chip8_display::DISPLAY_WIDTH / 8 {
            for b in 0..8 {
                let rect = SDL_Rect { x: ((j as i32) * 8 + b) * 10, y: (i as i32) * 10, w: 10, h: 10 };
                if device.display.display_data[i][j] & (0b10000000 >> b) > 0 {
                    SDL_RenderDrawRect(renderer, &rect);
                }
            }
        }
    }
}

fn main() {
    let mut device = chip8::build_chip8();
    load_program(device.borrow_mut(), "./resources/IBM");

    unsafe {
        assert_eq!(SDL_Init(SDL_INIT_EVERYTHING), 0);

        let window = SDL_CreateWindow(
            b"Khopa's Rusty Chip 8 Emulator\0".as_ptr().cast(),
            SDL_WINDOWPOS_CENTERED,
            SDL_WINDOWPOS_CENTERED,
            (chip8_display::DISPLAY_WIDTH * 10).try_into().unwrap(),
            (chip8_display::DISPLAY_HEIGHT * 10).try_into().unwrap(),
            (SDL_WINDOW_OPENGL | SDL_WINDOW_ALLOW_HIGHDPI).0,
        );
        // Panic if window is not null
        assert!(!window.is_null());

        let renderer = SDL_CreateRenderer(window, -1, 1);
        // Panic if renderer is not null
        assert!(!renderer.is_null());
        SDL_SetRenderDrawColor(renderer, 0, 0, 255, 255);

        let mut event = SDL_Event::default();
        loop {
            assert_eq!(SDL_WaitEvent(&mut event), 1);
            match event.type_ {
              SDL_QUIT => {
                println!("SDL_QUIT");
                break;
              }
              _ => (),
            }

            SDL_SetRenderDrawColor(renderer, 0, 0, 0, 255);
            SDL_RenderClear(renderer);
            render_chip8_display(renderer, &device);
            SDL_RenderPresent(renderer);

            // runs ~540 ops/s (540hz cpu speed)
            SDL_Delay(25); // 40 FPS cap
            for b in 0..14 {
                step(device.borrow_mut());
            }
        }

        SDL_DestroyWindow(window);
        SDL_Quit();
    }
}
