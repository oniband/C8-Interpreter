use std::fs::File;

const PATH_TO_FILE: &str = "./IBMLogo.ch8";

mod cpu;
use crate::cpu::{Cpu, Instruction};

fn main() -> std::io::Result<()> {
    let mut cpu = Cpu::new();
    let mut program = File::open(PATH_TO_FILE)?;
    cpu.load_program_into_memory(&mut program);

    while !cpu.temp_should_halt {
        let instruction: Instruction = cpu.fetch();
        cpu.decode_and_execute(instruction);
    }
    Ok(())
}
