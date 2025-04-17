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

    // KEYPAD //
    // ONE //
    if d.is_key_up(KeyboardKey::KEY_ONE) {
        d.draw_rectangle_lines(970, 50, 65, 65, Color::WHITE);
        d.draw_text("1", 995, 60, 50, Color::WHITE);
    } else {
        d.draw_rectangle(970, 45, 65, 65, Color::WHITE);
        d.draw_text("1", 995, 60, 50, Color::BLACK);
    }
    // TWO //
    if d.is_key_up(KeyboardKey::KEY_TWO) {
        d.draw_rectangle_lines(1035, 50, 65, 65, Color::WHITE);
        d.draw_text("2", 1060, 60, 50, Color::WHITE);
    } else {
        d.draw_rectangle(1035, 45, 65, 65, Color::WHITE);
        d.draw_text("2", 1060, 60, 50, Color::BLACK);
    }
    // THREE //
    if d.is_key_up(KeyboardKey::KEY_THREE) {
        d.draw_rectangle_lines(1100, 50, 65, 65, Color::WHITE);
        d.draw_text("3", 1125, 60, 50, Color::WHITE);
    } else {
        d.draw_rectangle(1100, 45, 65, 65, Color::WHITE);
        d.draw_text("3", 1125, 60, 50, Color::BLACK);
    }
    // C //
    if d.is_key_up(KeyboardKey::KEY_FOUR) {
        d.draw_rectangle_lines(1165, 50, 65, 65, Color::WHITE);
        d.draw_text("C", 1190, 60, 50, Color::WHITE);
    } else {
        d.draw_rectangle(1165, 45, 65, 65, Color::WHITE);
        d.draw_text("C", 1190, 60, 50, Color::BLACK);
    }
    // FOUR //
    if d.is_key_up(KeyboardKey::KEY_Q) {
        d.draw_rectangle_lines(970, 115, 65, 65, Color::WHITE);
        d.draw_text("4", 995, 125, 50, Color::WHITE);
    } else {
        d.draw_rectangle(970, 110, 65, 65, Color::WHITE);
        d.draw_text("4", 995, 125, 50, Color::BLACK);
    }
    // FIVE //
    if d.is_key_up(KeyboardKey::KEY_W) {
        d.draw_rectangle_lines(1035, 115, 65, 65, Color::WHITE);
        d.draw_text("5", 1060, 125, 50, Color::WHITE);
    } else {
        d.draw_rectangle(1035, 110, 65, 65, Color::WHITE);
        d.draw_text("5", 1060, 125, 50, Color::BLACK);
    }
    // SIX //
    if d.is_key_up(KeyboardKey::KEY_E) {
        d.draw_rectangle_lines(1100, 115, 65, 65, Color::WHITE);
        d.draw_text("6", 1125, 125, 50, Color::WHITE);
    } else {
        d.draw_rectangle(1100, 110, 65, 65, Color::WHITE);
        d.draw_text("6", 1125, 125, 50, Color::BLACK);
    }
    // D //
    if d.is_key_up(KeyboardKey::KEY_R) {
        d.draw_rectangle_lines(1165, 115, 65, 65, Color::WHITE);
        d.draw_text("D", 1190, 125, 50, Color::WHITE);
    } else {
        d.draw_rectangle(1165, 110, 65, 65, Color::WHITE);
        d.draw_text("D", 1190, 125, 50, Color::BLACK);
    }
    // SEVEN //
    if d.is_key_up(KeyboardKey::KEY_A) {
        d.draw_rectangle_lines(970, 180, 65, 65, Color::WHITE);
        d.draw_text("7", 995, 190, 50, Color::WHITE);
    } else {
        d.draw_rectangle(970, 175, 65, 65, Color::WHITE);
        d.draw_text("7", 995, 190, 50, Color::BLACK);
    }
    // EIGHT //
    if d.is_key_up(KeyboardKey::KEY_S) {
        d.draw_rectangle_lines(1035, 180, 65, 65, Color::WHITE);
        d.draw_text("8", 1060, 190, 50, Color::WHITE);
    } else {
        d.draw_rectangle(1035, 175, 65, 65, Color::WHITE);
        d.draw_text("8", 1060, 190, 50, Color::BLACK);
    }
    // NINE //
    if d.is_key_up(KeyboardKey::KEY_D) {
        d.draw_rectangle_lines(1100, 180, 65, 65, Color::WHITE);
        d.draw_text("9", 1125, 190, 50, Color::WHITE);
    } else {
        d.draw_rectangle(1100, 175, 65, 65, Color::WHITE);
        d.draw_text("9", 1125, 190, 50, Color::BLACK);
    }
    // E //
    if d.is_key_up(KeyboardKey::KEY_F) {
        d.draw_rectangle_lines(1165, 180, 65, 65, Color::WHITE);
        d.draw_text("E", 1190, 190, 50, Color::WHITE);
    } else {
        d.draw_rectangle(1165, 175, 65, 65, Color::WHITE);
        d.draw_text("E", 1190, 190, 50, Color::BLACK);
    }
    // A //
    if d.is_key_up(KeyboardKey::KEY_Z) {
        d.draw_rectangle_lines(970, 245, 65, 65, Color::WHITE);
        d.draw_text("A", 995, 255, 50, Color::WHITE);
    } else {
        d.draw_rectangle(970, 240, 65, 65, Color::WHITE);
        d.draw_text("A", 995, 255, 50, Color::BLACK);
    }
    // ZERO //
    if d.is_key_up(KeyboardKey::KEY_X) {
        d.draw_rectangle_lines(1035, 245, 65, 65, Color::WHITE);
        d.draw_text("0", 1060, 255, 50, Color::WHITE);
    } else {
        d.draw_rectangle(1035, 240, 65, 65, Color::WHITE);
        d.draw_text("0", 1060, 255, 50, Color::BLACK);
    }
    // B //
    if d.is_key_up(KeyboardKey::KEY_C) {
        d.draw_rectangle_lines(1100, 245, 65, 65, Color::WHITE);
        d.draw_text("B", 1125, 255, 50, Color::WHITE);
    } else {
        d.draw_rectangle(1100, 240, 65, 65, Color::WHITE);
        d.draw_text("B", 1125, 255, 50, Color::BLACK);
    }
    // F //
    if d.is_key_up(KeyboardKey::KEY_V) {
        d.draw_rectangle_lines(1165, 245, 65, 65, Color::WHITE);
        d.draw_text("F", 1190, 255, 50, Color::WHITE);
    } else {
        d.draw_rectangle(1165, 240, 65, 65, Color::WHITE);
        d.draw_text("F", 1190, 255, 50, Color::BLACK);
    }
    // KEYPAD //
}
