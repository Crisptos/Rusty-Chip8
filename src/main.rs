mod chip8;
use chip8::{Chip8, SPRITES};
use raylib::prelude::*;
use raylib::core::audio::{Sound, RaylibAudio};
use std::{thread, time::Duration};

fn process_input(d: &RaylibHandle, chip8: &mut Chip8){
    if d.is_key_down(KeyboardKey::KEY_ONE) {
        chip8.keyboard[0x01] = true;
    } else {
        chip8.keyboard[0x01] = false;
    }

    if d.is_key_down(KeyboardKey::KEY_TWO) {
        chip8.keyboard[0x02] = true;
    } else {
        chip8.keyboard[0x02] = false;
    }

    if d.is_key_down(KeyboardKey::KEY_THREE) {
        chip8.keyboard[0x03] = true;
    } else {
        chip8.keyboard[0x03] = false;
    }

    if d.is_key_down(KeyboardKey::KEY_FOUR) {
        chip8.keyboard[0x04] = true;
    } else {
        chip8.keyboard[0x04] = false;
    }

    if d.is_key_down(KeyboardKey::KEY_FIVE) {
        chip8.keyboard[0x05] = true;
    } else {
        chip8.keyboard[0x05] = false;
    }

    if d.is_key_down(KeyboardKey::KEY_SIX) {
        chip8.keyboard[0x06] = true;
    } else {
        chip8.keyboard[0x06] = false;
    }

    if d.is_key_down(KeyboardKey::KEY_SEVEN) {
        chip8.keyboard[0x07] = true;
    } else {
        chip8.keyboard[0x07] = false;
    }

    if d.is_key_down(KeyboardKey::KEY_EIGHT) {
        chip8.keyboard[0x08] = true;
    } else {
        chip8.keyboard[0x08] = false;
    }

    if d.is_key_down(KeyboardKey::KEY_NINE) {
        chip8.keyboard[0x09] = true;
    } else {
        chip8.keyboard[0x09] = false;
    }

    if d.is_key_down(KeyboardKey::KEY_ZERO) {
        chip8.keyboard[0xA] = true;
    } else {
        chip8.keyboard[0xA] = false;
    }

    if d.is_key_down(KeyboardKey::KEY_MINUS) {
        chip8.keyboard[0xB] = true;
    } else {
        chip8.keyboard[0xB] = false;
    }

    if d.is_key_down(KeyboardKey::KEY_EQUAL) {
        chip8.keyboard[0xC] = true;
    } else {
        chip8.keyboard[0xC] = false;
    }

    if d.is_key_down(KeyboardKey::KEY_P) {
        chip8.keyboard[0xD] = true;
    } else {
        chip8.keyboard[0xD] = false;
    }

    if d.is_key_down(KeyboardKey::KEY_LEFT_BRACKET) {
        chip8.keyboard[0xE] = true;
    } else {
        chip8.keyboard[0xE] = false;
    }

    if d.is_key_down(KeyboardKey::KEY_RIGHT_BRACKET) {
        chip8.keyboard[0xF] = true;
    } else {
        chip8.keyboard[0xF] = false;
    }
}

fn main() {
    let (mut rl, thread) = raylib::init().size(640, 320).title("Chip 8 DEV").build();
    let mut audio = RaylibAudio::init_audio_device();
    let beep = Sound::load_sound("audio/beep.wav").unwrap();
    let mut chip8 = Chip8::new();
    chip8.initialize();

    while !rl.window_should_close() {
        // Detect window close button or ESC key
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        // Check State of Hardware
        // Screen
        for x in 0..64 {
            for y in 0..32 {
                if chip8.get_pixel(x, y) {
                    d.draw_rectangle((x * 10) as i32, (y * 10) as i32, 10, 10, Color::RAYWHITE);
                }
            }
        }

        // Update Keyboard
        process_input(&d, &mut chip8);

        // Dispatch Next Opcode
        let opcode = chip8.get_next_op();
        println!("Opcode: {:#02x}\n __Registers__", opcode);
        chip8.dispatch(opcode);
        chip8.registers.print_status();

        // // Decrement Timers
        if chip8.registers.dt > 0 {
            thread::sleep(Duration::from_millis(5));
            chip8.registers.dt -= 1;
        }
        if chip8.registers.st > 0 {
            audio.play_sound_multi(&beep);
            chip8.registers.st -= 1;
        }
    }
}
