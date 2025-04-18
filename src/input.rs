use crate::cpu::Cpu;
use raylib::prelude::*;

pub fn poll_input(rl: &mut RaylibHandle, cpu: &mut Cpu) {
    if !rl.is_key_up(KeyboardKey::KEY_ONE) {
        cpu.current_key = 1;
    }
    // TWO //
    if !rl.is_key_up(KeyboardKey::KEY_TWO) {
        cpu.current_key = 2;
    }
    // THREE //
    if !rl.is_key_up(KeyboardKey::KEY_THREE) {
        cpu.current_key = 3;
    }
    // C //
    if !rl.is_key_up(KeyboardKey::KEY_FOUR) {
        cpu.current_key = 0xC;
    }
    // FOUR //
    if !rl.is_key_up(KeyboardKey::KEY_Q) {
        cpu.current_key = 4;
    }
    // FIVE //
    if !rl.is_key_up(KeyboardKey::KEY_W) {
        cpu.current_key = 5;
    }
    // SIX //
    if !rl.is_key_up(KeyboardKey::KEY_E) {
        cpu.current_key = 6;
    }
    // D //
    if !rl.is_key_up(KeyboardKey::KEY_R) {
        cpu.current_key = 0xD;
    }
    // SEVEN //
    if !rl.is_key_up(KeyboardKey::KEY_A) {
        cpu.current_key = 7;
    }
    // EIGHT //
    if !rl.is_key_up(KeyboardKey::KEY_S) {
        cpu.current_key = 8;
    }
    // NINE //
    if !rl.is_key_up(KeyboardKey::KEY_D) {
        cpu.current_key = 9;
    }
    // E //
    if !rl.is_key_up(KeyboardKey::KEY_F) {
        cpu.current_key = 0xE;
    }
    // A //
    if !rl.is_key_up(KeyboardKey::KEY_Z) {
        cpu.current_key = 0xA;
    }
    // ZERO //
    if !rl.is_key_up(KeyboardKey::KEY_X) {
        cpu.current_key = 0;
    }
    // B //
    if !rl.is_key_up(KeyboardKey::KEY_C) {
        cpu.current_key = 0xB;
    }
    // F //
    if rl.is_key_down(KeyboardKey::KEY_V) {
        cpu.current_key = 0xF;
    }
}
