use raylib::prelude::*;
use std::fs::File;
use std::time::{Duration, Instant};

mod cpu;
use crate::cpu::{Cpu, Instruction};

mod input;
use crate::input::poll_input;

mod graphics;
use crate::graphics::{WINDOW_HEIGHT, WINDOW_WIDTH, draw_game_pixels, draw_ui_elements};

mod util;
use crate::util::validate_args;

const FPS: u32 = 60;
fn main() -> std::io::Result<()> {
    let mut cpu = Cpu::new();

    match validate_args() {
        Ok(value) => {
            let mut program = File::open(value)?;
            cpu.load_program_into_memory(&mut program);
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
    rl.set_target_fps(FPS);
    rl.set_trace_log(TraceLogLevel::LOG_NONE);

    cpu.set_step_mode(false);
    let mut opcode_strings: [u16; 3] = Default::default();
    let mut timer = Instant::now();

    while !rl.window_should_close() {
        // A bis implementation of a cylce speed, this is about 2Mhz
        if timer.elapsed() >= Duration::from_millis(10) {
            timer = Instant::now();
            for _ in 0..=20 {
                if !cpu.should_halt {
                    poll_input(&mut rl, &mut cpu);
                    if !cpu.step_mode {
                        opcode_strings = cpu.fetch_opcodes();
                        let instruction: Instruction = cpu.fetch();
                        cpu.decode_and_execute(instruction);
                    } else if rl.is_key_pressed(KeyboardKey::KEY_SPACE) {
                        opcode_strings = cpu.fetch_opcodes();
                        let instruction: Instruction = cpu.fetch();
                        cpu.decode_and_execute(instruction);
                    }
                }
            }
        }

        let mut d = rl.begin_drawing(&thread);
        draw_ui_elements(&mut d, &mut cpu, opcode_strings);
        draw_game_pixels(&mut d, &cpu);
    }
    Ok(())
}
