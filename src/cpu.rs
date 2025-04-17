use std::fs::File;
use std::io::Read;
use std::usize;

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
    pub memory: [u8; 4096_usize],
    pub v_registers: [u8; 16],
    pub index_register: u16,
    pub program_counter: u16,
    stack: Vec<u16>,
    pub should_halt: bool,
    pub pixel_buffer: [[bool; 64]; 32],
    //The clock speed is what will determine raylibs FPS. Seems to be the easiest way to implement
    //a cycle speed since we're calling cpu functions from within the raylib game loop.
    pub clock_speed: u32,
    pub step_mode: bool,
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
            clock_speed: 60,
            step_mode: false,
        }
    }

    pub fn load_program_into_memory(&mut self, program: &mut File) {
        let mut data: Vec<u8> = Vec::new();
        //Traditionally, the interpreter was put into the first 512 bytes of memory, meaning that
        //ROMs had to fit into the remaining memory, we check that here
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
        self.set_program_counter(0x200); //Set it back to 512 which is the first instruction.
    }

    //Looks scary but we're just doing bitwise operations on each byte to extract 4 nibbles and a 12
    //bit memory address from the final 3 nibbles.
    pub fn fetch(&mut self) -> Instruction {
        let decoded_opcode: u16 = ((self.memory[self.program_counter as usize] as u16) << 8)
            | self.memory[(self.program_counter + 1) as usize] as u16;
        let decoded_instruction = Instruction {
            instruction: (self.memory[self.program_counter as usize] >> 4),
            x: (self.memory[self.program_counter as usize] & 0x0F),
            y: (self.memory[(self.program_counter + 1) as usize] >> 4),
            n: (self.memory[(self.program_counter + 1) as usize] & 0x0F),
            nn: self.memory[(self.program_counter + 1) as usize],
            nnn: decoded_opcode & 0x0FFF,
        };
        self.increment_program_counter(2);
        decoded_instruction
    }

    // Creates a string representation of the Current, Previous and Next instruction in memory so
    // that we can show it in the UI
    pub fn fetch_opcodes(&mut self) -> [u16; 3] {
        let opcode_previous: u16 = ((self.memory[self.program_counter as usize - 2] as u16) << 8)
            | self.memory[(self.program_counter - 1) as usize] as u16;

        let opcode_current: u16 = ((self.memory[self.program_counter as usize] as u16) << 8)
            | self.memory[(self.program_counter + 1) as usize] as u16;

        let opcode_next: u16 = ((self.memory[self.program_counter as usize + 2] as u16) << 8)
            | self.memory[(self.program_counter + 3) as usize] as u16;

        [opcode_previous, opcode_current, opcode_next]
    }

    pub fn decode_and_execute(&mut self, instruction: Instruction) {
        match instruction.instruction {
            0x0 => {
                // We need to differenciate between 0x00E0(clear screen) and 0x00EE(return from subroutine)
                if instruction.y == 0xE && instruction.n != 0xE {
                    println!("CLS");
                    self.pixel_buffer = [[false; 64]; 32];
                }
                if instruction.n == 0xE && instruction.n == 0x0E {
                    if let Some(return_address) = self.stack.pop() {
                        println!("RET {return_address}");
                        self.set_program_counter(return_address);
                    } else {
                        panic!("Tried to return from a subroutine with an empty Stack!");
                    }
                }
            }
            0x1 => {
                println!("JMP {}", instruction.nnn);

                //Roms have a tendency to have a "JUMP TO CURRENT INSTRUCTION" at the end of their instructions
                //They do this because there's no "stop execution" instruction
                //Here we make sure we're not just looping forever at the end
                if self.program_counter - 2 == instruction.nnn {
                    println!("Infinte loop detected, halting execution!");
                    self.clock_speed = 10;
                    self.should_halt = true;
                }

                self.set_program_counter(instruction.nnn);
            }
            0x2 => {
                println!("CALL {}", instruction.nnn);
                self.stack.push(self.program_counter);
                self.set_program_counter(instruction.nnn);
            }
            0x3 => {
                println!(
                    "JMP IF EQUAL V{} {}, {}",
                    instruction.x, self.v_registers[instruction.x as usize], instruction.nnn
                );
                if self.v_registers[instruction.x as usize] == instruction.nn {
                    self.increment_program_counter(2);
                }
            }
            0x4 => {
                if self.v_registers[instruction.x as usize] != instruction.nn {
                    self.increment_program_counter(2);
                }
            }
            0x5 => {
                if self.v_registers[instruction.x as usize]
                    == self.v_registers[instruction.y as usize]
                {
                    self.increment_program_counter(2);
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
            0x8 => match instruction.n {
                0x0 => {
                    println!("MOV V{}, V{}", instruction.x, instruction.y);
                    self.v_registers[instruction.x as usize] =
                        self.v_registers[instruction.y as usize];
                }
                0x1 => {
                    println!("OR V{}, V{}", instruction.x, instruction.y);
                    self.v_registers[instruction.x as usize] = self.v_registers
                        [instruction.x as usize]
                        | self.v_registers[instruction.y as usize];
                }
                0x2 => {
                    println!("AND V{}, V{}", instruction.x, instruction.y);
                    self.v_registers[instruction.x as usize] = self.v_registers
                        [instruction.x as usize]
                        & self.v_registers[instruction.y as usize];
                }
                0x3 => {
                    println!("XOR V{}, V{}", instruction.x, instruction.y);
                    self.v_registers[instruction.x as usize] = self.v_registers
                        [instruction.x as usize]
                        ^ self.v_registers[instruction.y as usize];
                }
                0x4 => {
                    println!("ADD V{}, V{}", instruction.x, instruction.y);

                    //This instruction requires that if there was an overflow, we set the VF register to one
                    //Otherwise it gets set to zero
                    let (_, overflowed) = self.v_registers[instruction.x as usize]
                        .overflowing_add(self.v_registers[instruction.y as usize]);
                    if overflowed {
                        self.v_registers[0xF] = 1;
                    } else {
                        self.v_registers[0xF] = 0;
                    }

                    self.v_registers[instruction.x as usize] = self.v_registers
                        [instruction.x as usize]
                        .wrapping_add(self.v_registers[instruction.y as usize])
                }
                0x5 => {
                    println!(
                        "SUB V{}, V{} MINUS V{}",
                        instruction.x, instruction.x, instruction.y
                    );

                    // If we're going to underflow, vf should be set to 1
                    if self.v_registers[instruction.x as usize]
                        > self.v_registers[instruction.y as usize]
                    {
                        self.v_registers[0xF] = 1;
                    } else if self.v_registers[instruction.x as usize]
                        < self.v_registers[instruction.y as usize]
                    {
                        self.v_registers[0xF] = 0;
                    }

                    self.v_registers[instruction.x as usize] = self.v_registers
                        [instruction.x as usize]
                        .wrapping_sub(self.v_registers[instruction.y as usize]);
                }
                0x6 => {
                    println!(
                        "MOV V{}, V{} >> 1",
                        self.v_registers[instruction.x as usize],
                        self.v_registers[instruction.y as usize]
                    );

                    //We have to set the VF register to the bit that was shifted out
                    if self.v_registers[instruction.y as usize] & 1 != 0 {
                        self.v_registers[0xF] = 1;
                    } else {
                        self.v_registers[0xF] = 0;
                    }

                    self.v_registers[instruction.x as usize] =
                        self.v_registers[instruction.y as usize] >> 1;
                }
                0x7 => {
                    println!(
                        "SUB V{}, V{} MINUS V{}",
                        instruction.x, instruction.y, instruction.x
                    );

                    // If we're going to underflow, vf should be set to 1
                    if self.v_registers[instruction.y as usize]
                        > self.v_registers[instruction.x as usize]
                    {
                        self.v_registers[0xF] = 1;
                    } else if self.v_registers[instruction.y as usize]
                        < self.v_registers[instruction.x as usize]
                    {
                        self.v_registers[0xF] = 0;
                    }

                    self.v_registers[instruction.x as usize] = self.v_registers
                        [instruction.y as usize]
                        .wrapping_sub(self.v_registers[instruction.x as usize]);
                }
                0xE => {
                    println!(
                        "MOV V{}, V{} << 1",
                        self.v_registers[instruction.x as usize],
                        self.v_registers[instruction.y as usize]
                    );

                    //We have to set the VF register to the bit that was shifted out
                    if self.v_registers[instruction.y as usize] & 1 != 0 {
                        self.v_registers[0xF] = 1;
                    } else {
                        self.v_registers[0xF] = 0;
                    }

                    self.v_registers[instruction.x as usize] =
                        self.v_registers[instruction.y as usize] << 1;
                }
                _ => (),
            },
            0x9 => {
                if self.v_registers[instruction.x as usize]
                    != self.v_registers[instruction.y as usize]
                {
                    self.increment_program_counter(2);
                }
            }
            0xA => {
                println!("MOV I, {}", instruction.nnn);
                self.index_register = instruction.nnn;
            }
            0xB => {
                println!("JMP {}", (instruction.nnn + self.v_registers[0x0] as u16));
                self.set_program_counter(instruction.nnn + self.v_registers[0x0] as u16);
            }
            0xC => {
                //This should generate a random number, place it into x register
                //and binary AND it with nn
                //Need to look into random number solutions.
                todo!();
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
                    let sprite_data: u8 = self.memory[(index + row as u16) as usize];
                    //The X coordinate should be reset for each row that we do
                    let mut x: usize = (self.v_registers[instruction.x as usize] % 64).into();
                    for bit in (0..8).rev() {
                        if sprite_data & (1 << bit) != 0 && self.pixel_buffer[y][x] {
                            self.v_registers[0xF] = 1
                        }
                        //Pixels are XOR'd onto the screen here,
                        //If it's anything other than 0, it means the current bit has been set
                        self.pixel_buffer[y][x] ^= sprite_data & (1 << bit) != 0;

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
            0xF => match instruction.nn {
                0x1E => {
                    println!("ADD I, V{}", instruction.x);
                    self.index_register += self.v_registers[instruction.x as usize] as u16;
                }
                0x33 => {
                    println!("CONV V{}", instruction.x);
                    //This is probably the coolest instruction, you convert the binary value
                    //into a decimal and then add all the digits together
                    let value = self.v_registers[instruction.x as usize];
                    self.memory[self.index_register as usize] = value / 100;
                    self.memory[self.index_register as usize + 1] = (value / 10) % 10;
                    self.memory[self.index_register as usize + 2] = value % 10;
                }
                0x55 => {
                    println!("MEM SET FROM {} FOR {}", self.index_register, instruction.x);
                    for register in 0..=instruction.x {
                        self.memory[(self.index_register as usize) + register as usize] =
                            self.v_registers[0 + register as usize];
                    }
                }
                0x65 => {
                    println!(
                        "MEM GRAB FROM {} FOR {}",
                        self.index_register, instruction.x
                    );
                    for register in 0..=instruction.x {
                        self.v_registers[0 + register as usize] =
                            self.memory[(self.index_register as usize) + register as usize];
                    }
                }
                _ => {}
            },
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

    pub fn set_step_mode(&mut self, value: bool) {
        self.step_mode = value;
    }

    //A simple debug function that will dump out the contents of the pixel buffer into a
    //*hopefully* more readable format. I find it useful to compare this to the actual pixel data
    //from the ROM since it looks exactly like the sprite it represents except with 1's and 0's
    //What a cool format.
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
