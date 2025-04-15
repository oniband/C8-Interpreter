use bitvec::prelude::*;
use std::fs::File;
use std::io::Read;

#[derive(Debug)]
pub struct Instruction {
    pub instruction: u8,
    pub x: u8,
    pub y: u8,
    pub n: u8,
    pub nn: u8,
    pub nnn: u16,
}

pub struct Cpu {
    memory: [u8; 4096_usize],
    v_registers: [u8; 16],
    index_register: u16,
    program_counter: u16,
    stack: Vec<u16>,
    pub should_halt: bool,
    pub pixel_buffer: [[bool; 64]; 32],
    //The clock speed is what will determine raylibs FPS. Seems to be the easiest way to implement
    //a cycle speed since we're calling cpu functions from within the raylib game loop.
    pub clock_speed: u32,
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            memory: [0; 4096_usize],
            v_registers: [0; 16],
            index_register: 0,
            program_counter: 0x200,
            stack: Vec::new(),
            should_halt: false,
            pixel_buffer: [[false; 64]; 32],
            clock_speed: 30,
        }
    }

    pub fn load_program_into_memory(&mut self, program: &mut File) {
        let mut data: Vec<u8> = Vec::new();
        // Traditionally, the interpreter was put into the first 512 bytes of memory, meaning that
        // ROMs had to fit into the remaining memory, we check that here
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
        self.set_program_counter(0x200); // set it back to 512 which is the first instruction.
    }

    // Looks scary but we're just doing bitwise operations on each byte to extract 4 nibbles and a 12
    // bit memory address from the final 3 nibbles.
    pub fn fetch(&mut self) -> Instruction {
        let opcode: u16 = ((self.memory[self.program_counter as usize] as u16) << 8)
            | self.memory[(self.program_counter + 1) as usize] as u16;
        let decoded_instruction = Instruction {
            instruction: (self.memory[self.program_counter as usize] >> 4),
            x: (self.memory[self.program_counter as usize] & 0x0F),
            y: (self.memory[(self.program_counter + 1) as usize] >> 4),
            n: (self.memory[(self.program_counter + 1) as usize] & 0x0F),
            nn: self.memory[(self.program_counter + 1) as usize],
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
                    self.pixel_buffer = [[false; 64]; 32];
                }
            }
            0x1 => {
                println!("JMP {}", instruction.nnn);
                self.set_program_counter(instruction.nnn);

                // Roms have a tendency to have a "JUMP TO SELF" at the end of their instructions
                // They do this because there's no "stop execution" instruction.
                // Here we make sure we're not just looping forever at the end.
                if self.program_counter == instruction.nnn {
                    println!("Infinte loop detected, halting execution!");
                    self.should_halt = true;
                    self.clock_speed = 0;
                }
            }
            0x6 => {
                println!("MOV V{}, {}", instruction.x, instruction.nn);
                self.v_registers[instruction.x as usize] = instruction.nn;
            }
            0x7 => {
                println!("ADD V{}, {}", instruction.x, instruction.nn);
                self.v_registers[instruction.x as usize] =
                    self.v_registers[instruction.x as usize].wrapping_add(instruction.nn);
            }
            0xA => {
                println!("MOV I, {}", instruction.nnn);
                self.index_register = instruction.nnn;
            }
            0xD => {
                println!(
                    "DRAW FROM X:{} Y:{} for {} rows",
                    instruction.x, instruction.y, instruction.n
                );
                let index = self.index_register;
                //The Y coordinate doesn't need to be reset, we can initialize it outside the loop
                let mut y: usize = (self.v_registers[instruction.y as usize] % 32).into();
                self.v_registers[0xF] = 0;

                for row in 0..instruction.n {
                    let mut sprite_data: u8 = self.memory[(index + row as u16) as usize];
                    //The X coordinate should be reset for each row that we do
                    let mut x: usize = (self.v_registers[instruction.x as usize] % 64).into();
                    for bit in sprite_data.view_bits_mut::<Msb0>().iter() {
                        if *bit && self.pixel_buffer[y][x] {
                            self.v_registers[0xF] = 1
                        }
                        // Pixels are XOR'd onto the screen here,
                        self.pixel_buffer[y][x] ^= *bit;

                        x += 1;
                        if x > 63 {
                            x = 0
                        }
                    }
                    y += 1;
                    if y > 31 {
                        y = 0
                    }
                }
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

    // A simple debug function that will dump out the contents of the pixel buffer into a
    // *hopefully* more readable format. I find it useful to compare this to the actual pixel data
    // from the ROM since it looks exactly like the sprite it represents except with 1's and 0's
    // What a cool format.
    fn _dump_pixel_buffer(&self) {
        for y in 0..32 {
            print!("{{");
            for x in 0..64 {
                if self.pixel_buffer[y][x] {
                    print!("1 ");
                } else {
                    print!("0 ");
                }
            }
            println!("}}");
        }
    }
}
