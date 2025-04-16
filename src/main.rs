use raylib::prelude::*;
use std::fs::File;

mod util;
use crate::util::validate_args;

mod cpu;
use crate::cpu::{Cpu, Instruction};

const WINDOW_WIDTH: i32 = 640;
const WINDOW_HEIGHT: i32 = 320;

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
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .title("C8-Emu")
        .build();
    rl.set_target_fps(cpu.clock_speed);

    while !rl.window_should_close() {
        if !cpu.should_halt {
            let instruction: Instruction = cpu.fetch();
            cpu.decode_and_execute(instruction);
        }

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::ORANGE);
        for y in 0..32 {
            for x in 0..64 {
                if cpu.pixel_buffer[y][x] {
                    d.draw_rectangle(x as i32 * 10, y as i32 * 10, 10, 10, Color::YELLOW);
                }
            }
        }
    }
    Ok(())
}
