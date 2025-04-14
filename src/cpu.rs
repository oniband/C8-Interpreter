use std::fs::File;
use std::io::Read;

pub struct Cpu {
    memory: [u8; 4096 as usize],
    v_registers: [u8; 16],
    index_register: u16,
    program_counter: u16,
    stack: Vec<u16>,
    stack_pointer: u16,
    pub temp_should_halt: bool,
}
#[derive(Debug)]
pub struct Instruction {
    pub instruction: u8,
    pub x: u8,
    pub y: u8,
    pub n: u8,
    pub nn: u8,
    pub nnn: u16,
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            memory: [0; 4096 as usize],
            v_registers: [0; 16],
            index_register: 0,
            program_counter: 0x200,
            stack: Vec::new(),
            stack_pointer: 0,
            temp_should_halt: false,
        }
    }

    pub fn load_program_into_memory(&mut self, program: &mut File) {
        let mut data: Vec<u8> = Vec::new();
        match program.read_to_end(&mut data) {
            Ok(value) => {
                if value > 3584 {
                    panic!(
                        "Memory Violation! Attempted to write {} bytes into memory from {:#04x}",
                        value, self.program_counter
                    );
                } else {
                    println!("Loading program with size of {value} bytes into memory");
                }
            }
            Err(err) => eprintln!("{err:?}"),
        }
        for byte in data {
            self.memory[self.program_counter as usize] = byte;
            self.increment_program_counter(1);
        }
        self.set_program_counter(0x200);
    }

    pub fn fetch(&mut self) -> Instruction {
        let opcode: u16 = (((self.memory[self.program_counter as usize] as u16) << 8)
            | self.memory[(self.program_counter + 1) as usize] as u16)
            .into();
        let decoded_instruction = Instruction {
            instruction: (self.memory[self.program_counter as usize] >> 4),
            x: (self.memory[self.program_counter as usize] & 0x0F),
            y: (self.memory[(self.program_counter + 1) as usize] >> 4),
            n: (self.memory[(self.program_counter + 1) as usize] & 0x0F),
            nn: self.memory[self.program_counter as usize],
            nnn: opcode & 0x0FFF,
        };
        self.increment_program_counter(2);
        decoded_instruction
    }

    pub fn decode_and_execute(&mut self, instruction: Instruction) {
        match instruction.instruction {
            0x0 => {
                if instruction.y == 0xE {
                    println!("CLS");
                }
            }
            0x1 => {
                println!("JMP {}", instruction.nnn);
                self.set_program_counter(instruction.nnn);

                if self.program_counter == instruction.nnn {
                    println!("Infinte loop detected, halting execution!");
                    self.temp_should_halt = true;
                }
            }
            0x6 => {
                println!("MOV V{}, {}", instruction.x, instruction.nn);
                self.v_registers[instruction.x as usize] = instruction.nn;
            }
            0x7 => {
                println!("ADD V{}, {}", instruction.x, instruction.nn);
                let _ = self.v_registers[instruction.x as usize].wrapping_add(instruction.nn);
            }
            0xA => {
                println!("MOV I, {}", instruction.nnn);
                self.index_register = instruction.nnn;
            }
            0xD => {
                println!("DRAW");
            }
            _ => println!("Instruction Unimplemented"),
        }
    }

    fn increment_program_counter(&mut self, value: u16) {
        if (self.program_counter + value) > 4095 {
            println!(
                "Overflowing Program counter from {} to {}",
                self.program_counter,
                (self.program_counter + value) - 4096
            );
            self.program_counter = (self.program_counter + value) - 4096;
        } else {
            self.program_counter += value;
        }
    }

    fn set_program_counter(&mut self, value: u16) {
        if value > 4095 {
            panic!(
                "Memory Violation! Atempted to set program counter to {value} with a memory size of 4096 bytes.",
            );
        } else {
            self.program_counter = value;
        }
    }
}
