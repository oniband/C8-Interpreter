use raylib::prelude::*;
use std::fs::File;

mod cpu;
use crate::cpu::{Cpu, Instruction};

const PATH_TO_FILE: &str = "./test_bin/IBMLogo.ch8";

fn main() -> std::io::Result<()> {
    let mut cpu = Cpu::new();
    let mut program = File::open(PATH_TO_FILE)?;
    cpu.load_program_into_memory(&mut program);

    while !cpu.temp_should_halt {
        let instruction: Instruction = cpu.fetch();
        cpu.decode_and_execute(instruction);
    }

    let (mut rl, thread) = raylib::init().size(650, 320).title("C8-Emu").build();
    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::BLACK);
    }
    Ok(())
}
