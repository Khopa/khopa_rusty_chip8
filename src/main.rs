#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(clippy::single_match)]

use std::{env, thread, time};
use std::borrow::{Borrow, BorrowMut};
use std::convert::TryInto;
use std::thread::sleep;
use std::time::Duration;

use clap::Parser;
use fermium::{
    *,
    error::*, events::*, prelude::*, video::*,
};
use fermium::prelude::{SDL_Rect, SDL_RenderDrawRectF};
use fermium::renderer::SDL_RenderDrawRect;
use rodio::{Decoder, OutputStream, Sink};
use rodio::source::{SineWave, Source};

use crate::chip8::{KEYBOARD_SIZE, load_program, step};
use crate::chip8_keyboard_utils::on_keyboard_event;
use crate::debug_utils::{print_display, print_keyboard, print_memory, print_registers};

mod chip8;
mod chip8_display;
mod chip8_memory;
mod debug_utils;
mod chip8_instructions;
mod chip8_keyboard_utils;

unsafe fn render_chip8_display(renderer: *mut SDL_Renderer, device: &chip8::Chip8) {
    for i in 0..chip8_display::DISPLAY_HEIGHT {
        for j in 0..chip8_display::DISPLAY_WIDTH / 8 {
            for b in 0..8 {
                let rect = SDL_Rect { x: ((j as i32) * 8 + b) * 10, y: (i as i32) * 10, w: 10, h: 10 };
                if device.display.display_data[i][j] & (0b10000000 >> b) > 0 {
                    SDL_SetRenderDrawColor(renderer, 90, 190, 90, 255);
                    SDL_RenderFillRect(renderer, &rect);
                }
                SDL_SetRenderDrawColor(renderer, 14, 48, 68, 255);
                SDL_RenderDrawRect(renderer, &rect);
            }
        }
    }
}

/// Chip 8 Emulator
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
   /// Rom file to load
   #[clap(short, long, value_parser)]
   filename: String,
}

fn main() {
    let args = Args::parse();

    let mut device = chip8::build_chip8();
    load_program(device.borrow_mut(), &args.filename);

    let mut window;
    let mut renderer;
    let mut event = SDL_Event::default();

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();
    let source = SineWave::new(500.0).amplify(0.20);
    sink.append(source);

    unsafe {
        assert_eq!(SDL_Init(SDL_INIT_EVERYTHING), 0);
        window = SDL_CreateWindow(
            b"Khopa's Rusty Chip 8 Emulator\0".as_ptr().cast(),
            SDL_WINDOWPOS_CENTERED,
            SDL_WINDOWPOS_CENTERED,
            (chip8_display::DISPLAY_WIDTH * 10).try_into().unwrap(),
            (chip8_display::DISPLAY_HEIGHT * 10).try_into().unwrap(),
            (SDL_WINDOW_OPENGL | SDL_WINDOW_ALLOW_HIGHDPI).0,
        );
        // Panic if window is not null
        assert!(!window.is_null());

        renderer = SDL_CreateRenderer(window, -1, 1);
        // Panic if renderer is not null
        assert!(!renderer.is_null());
        SDL_SetRenderDrawColor(renderer, 0, 0, 255, 255);
    }

    loop {

        let mut should_exit = false;

        unsafe {
            while SDL_PollEvent(&mut event) > 0 {
                match event.type_ {
                    SDL_QUIT => {
                        should_exit = true;
                    }
                    SDL_KEYUP => {
                        on_keyboard_event(&mut device, event.key.keysym.scancode, false);
                    }
                    SDL_KEYDOWN => {
                        on_keyboard_event(&mut device, event.key.keysym.scancode, true);
                    }
                    _ => (),
                }
            }
        }

        if(should_exit){
            break;
        }

        unsafe {
            SDL_SetRenderDrawColor(renderer, 7, 38, 54, 255);
            SDL_RenderClear(renderer);
            render_chip8_display(renderer, &device);
            SDL_RenderPresent(renderer);
            // runs ~540 ops/s (540hz cpu speed)
            SDL_Delay(25); // 40 FPS cap
        }

        let mut speed = 14;
        if device.turbo{
            speed = 140;
        }
        for b in 0..speed {
            step(device.borrow_mut());
            //print_registers(&device);
        }

        if device.st > 0 {
            sink.play();
        }else{
            sink.pause();
        }

        device.key = KEYBOARD_SIZE + 1;

    }

    unsafe{
        SDL_DestroyWindow(window);
        SDL_Quit();
    }
}
