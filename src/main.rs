use raylib::prelude::*;
use std::fs::File;

mod cpu;
use crate::cpu::{Cpu, Instruction};

mod graphics;
use crate::graphics::{WINDOW_HEIGHT, WINDOW_WIDTH, draw_game_pixels, draw_ui_elements};

mod util;
use crate::util::validate_args;

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
    rl.set_target_fps(cpu.clock_speed);
    cpu.set_step_mode(false);
    rl.set_trace_log(TraceLogLevel::LOG_NONE);
    let mut opcode_strings: [u16; 3] = Default::default();
    // rl.set_exit_key(Some(KeyboardKey::KEY_X));
    while !rl.window_should_close() {
        rl.set_target_fps(cpu.clock_speed);
        if !cpu.should_halt && !cpu.waiting_for_input {
            if !cpu.step_mode {
                opcode_strings = cpu.fetch_opcodes();
                let instruction: Instruction = cpu.fetch();
                cpu.decode_and_execute(instruction);
            } else {
                if rl.is_key_pressed(KeyboardKey::KEY_SPACE) {
                    opcode_strings = cpu.fetch_opcodes();
                    let instruction: Instruction = cpu.fetch();
                    cpu.decode_and_execute(instruction);
                }
            }
        }

        let mut d = rl.begin_drawing(&thread);
        draw_ui_elements(&mut d, &cpu, opcode_strings);
        draw_game_pixels(&mut d, &cpu);
    }
    Ok(())
}
