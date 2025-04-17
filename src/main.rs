use raylib::prelude::*;
use std::fs::File;

mod cpu;
use crate::cpu::{Cpu, Instruction};

mod util;
use crate::util::{_create_instruction_representation, validate_args};

const WINDOW_WIDTH: i32 = 1250;
const WINDOW_HEIGHT: i32 = 500;

const GAME_WIDTH: f32 = 640.0;
const GAME_HEIGHT: f32 = 320.0;
const GAME_PIXEL_SCALE_FACTOR: i32 = 10;

fn main() -> std::io::Result<()> {
    let mut cpu = Cpu::new();
    match validate_args() {
        Ok(value) => {
            let mut program = File::open(value)?;
            cpu.load_program_into_memory(&mut program);
            _create_instruction_representation(&mut program);
        }
        Err(err) => {
            panic!("{err}");
        }
    }

    let (mut rl, thread) = raylib::init()
        .width(WINDOW_WIDTH)
        .height(WINDOW_HEIGHT)
        .title("C8-Emu")
        .build();
    rl.set_target_fps(cpu.clock_speed);
    // rl.set_exit_key(Some(KeyboardKey::KEY_X));
    while !rl.window_should_close() {
        if !cpu.should_halt {
            let instruction: Instruction = cpu.fetch();
            cpu.decode_and_execute(instruction);
        }

        let mut d = rl.begin_drawing(&thread);
        // UI RENDERING //
        d.draw_rectangle(0, 0, 320, WINDOW_HEIGHT, Color::BLACK); // Left Black Background
        d.draw_rectangle(950, 0, 320, WINDOW_HEIGHT, Color::BLACK); // RIght Black Background
        d.draw_rectangle(320, 0, 640, 100, Color::BLACK); // Top Border
        d.draw_rectangle(320, WINDOW_HEIGHT - 100, 640, 100, Color::BLACK); // Bottom Border
        d.draw_rectangle_lines(115, 20, 60, 150, Color::WHITE); // Instruction List Box
        d.draw_rectangle_lines(115, 75, 60, 40, Color::WHITE); // Current Instruction Box
        d.draw_rectangle_lines(35, 75, 60, 40, Color::WHITE); // Program Counter Box
        d.draw_text(
            &format!("{}", cpu.program_counter),
            45,
            86,
            20,
            Color::WHITE,
        ); // Program Counter Value
        // V REGISTER RENDERING //
        let mut row: i32 = 0;
        let mut count: i32 = 1;
        let mut offset: i32 = 0;
        for register in 0..=15 {
            d.draw_rectangle_lines(20 + offset, 225 + row, 40, 40, Color::WHITE); // V Register Box
            d.draw_text(
                &format!("0X{:02x}", cpu.v_registers[register]),
                23 + offset,
                240 + row,
                15,
                Color::WHITE,
            ); // V Register Value
            offset += 70;
            if count % 4 == 0 {
                row += 70;
                offset = 0;
            }
            count += 1;
        }
        // V REGISTER RENDERING //
        // UI RENDERING //

        // GAME OUTPUT //
        d.clear_background(Color::DARKBLUE);
        for y in 0..32 {
            for x in 0..64 {
                if cpu.pixel_buffer[y][x] {
                    d.draw_rectangle(
                        (x as i32 * GAME_PIXEL_SCALE_FACTOR) + (WINDOW_WIDTH / 2)
                            - (GAME_WIDTH as i32 / 2),
                        (y as i32 * GAME_PIXEL_SCALE_FACTOR) + (WINDOW_HEIGHT / 2)
                            - (GAME_HEIGHT as i32 / 2),
                        GAME_PIXEL_SCALE_FACTOR,
                        GAME_PIXEL_SCALE_FACTOR,
                        Color::LIGHTBLUE,
                    );
                }
            }
        }
        // GAME OUTPUT //
    }
    Ok(())
}
