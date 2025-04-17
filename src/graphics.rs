use raylib::prelude::*;

use crate::cpu::Cpu;

pub const WINDOW_WIDTH: i32 = 1250;
pub const WINDOW_HEIGHT: i32 = 500;

const GAME_WIDTH: f32 = 640.0;
const GAME_HEIGHT: f32 = 320.0;
const GAME_PIXEL_SCALE_FACTOR: i32 = 10;

pub fn draw_game_pixels(d: &mut RaylibDrawHandle, cpu: &Cpu) {
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
}

pub fn draw_ui_elements(d: &mut RaylibDrawHandle, cpu: &Cpu, opcode_strings: [u16; 3]) {
    d.draw_rectangle(0, 0, 305, WINDOW_HEIGHT, Color::BLACK); // Left Black Background
    d.draw_rectangle(945, 0, 320, WINDOW_HEIGHT, Color::BLACK); // Right Black Background
    d.draw_rectangle(305, 0, 640, 90, Color::BLACK); // Top Border
    d.draw_rectangle(305, WINDOW_HEIGHT - 90, 640, 100, Color::BLACK); // Bottom Border
    // Current, next and previous instruction list
    d.draw_rectangle_lines(115, 35, 60, 120, Color::WHITE); // Instruction List Box
    d.draw_rectangle_lines(115, 75, 60, 40, Color::WHITE); // Current Instruction Box
    d.draw_text(
        &format!("{:04X}", &opcode_strings[0]),
        122,
        45,
        20,
        Color::WHITE,
    ); // Prev Instruction
    d.draw_text(
        &format!("{:04X}", &opcode_strings[1]),
        122,
        85,
        20,
        Color::WHITE,
    ); //Current Instruction
    d.draw_text(
        &format!("{:04X}", &opcode_strings[2]),
        122,
        125,
        20,
        Color::WHITE,
    ); // Next Instruction
    // Current, next and previous instruction list
    d.draw_text("PC", 50, 50, 20, Color::WHITE); // Program Counter Label
    d.draw_rectangle_lines(35, 75, 60, 40, Color::WHITE); // Program Counter Box
    d.draw_text(
        &format!("{}", cpu.program_counter),
        50,
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
            &format!("{:02X}", cpu.v_registers[register]),
            28 + offset,
            236 + row,
            20,
            Color::WHITE,
        ); // V Register Value
        d.draw_text(
            &format!("V{register:01X}",),
            25 + offset,
            205 + row,
            20,
            Color::WHITE,
        ); // V Register Label
        offset += 70;
        if count % 4 == 0 {
            row += 70;
            offset = 0;
        }
        count += 1;
    }
    // V REGISTER RENDERING //
}
