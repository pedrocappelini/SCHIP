use macroquad::prelude::{KeyCode, is_key_down};

use super::Schip;

pub fn update(chip8: &mut Schip) {
    chip8.keypad.fill(0);

    // 1 2 3 4 -> 1 2 3 C
    if is_key_down(KeyCode::Key1) {
        chip8.keypad[0x1] = 1;
    }
    if is_key_down(KeyCode::Key2) {
        chip8.keypad[0x2] = 1;
    }
    if is_key_down(KeyCode::Key3) {
        chip8.keypad[0x3] = 1;
    }
    if is_key_down(KeyCode::Key4) {
        chip8.keypad[0xC] = 1;
    }

    // Q W E R -> 4 5 6 D
    if is_key_down(KeyCode::Q) {
        chip8.keypad[0x4] = 1;
    }
    if is_key_down(KeyCode::W) {
        chip8.keypad[0x5] = 1;
    }
    if is_key_down(KeyCode::E) {
        chip8.keypad[0x6] = 1;
    }
    if is_key_down(KeyCode::R) {
        chip8.keypad[0xD] = 1;
    }

    // A S D F -> 7 8 9 E
    if is_key_down(KeyCode::A) {
        chip8.keypad[0x7] = 1;
    }
    if is_key_down(KeyCode::S) {
        chip8.keypad[0x8] = 1;
    }
    if is_key_down(KeyCode::D) {
        chip8.keypad[0x9] = 1;
    }
    if is_key_down(KeyCode::F) {
        chip8.keypad[0xE] = 1;
    }

    // Z X C V -> A 0 B F
    if is_key_down(KeyCode::Z) {
        chip8.keypad[0xA] = 1;
    }
    if is_key_down(KeyCode::X) {
        chip8.keypad[0x0] = 1;
    }
    if is_key_down(KeyCode::C) {
        chip8.keypad[0xB] = 1;
    }
    if is_key_down(KeyCode::V) {
        chip8.keypad[0xF] = 1;
    }
}
