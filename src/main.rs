use raylib::prelude::*;
use std::fs::File;

mod cpu;
use crate::cpu::{Cpu, Instruction};
// use crate::graphics::{generate_pixel_buffer};

const PATH_TO_FILE: &str = "./test_bin/IBMLogo.ch8";
const WINDOW_WIDTH: i32 = 640;
const WINDOW_HEIGHT: i32 = 320;

fn main() -> std::io::Result<()> {
    let mut cpu = Cpu::new();
    let mut program = File::open(PATH_TO_FILE)?;
    cpu.load_program_into_memory(&mut program);

    let (mut rl, thread) = raylib::init()
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .title("C8-Emu")
        .build();
    rl.set_target_fps(60);
    while !rl.window_should_close() {
        if !cpu.temp_should_halt {
            if rl.is_key_pressed(KeyboardKey::KEY_SPACE) {
                let instruction: Instruction = cpu.fetch();
                cpu.decode_and_execute(instruction);
            }
        }

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        for y in 0..32 {
            for x in 0..64 {
                if cpu.pixel_buffer[y][x] {
                    d.draw_rectangle(x as i32 * 10, y as i32 * 10, 10, 10, Color::WHITE);
                }
            }
        }
    }
    Ok(())
}
