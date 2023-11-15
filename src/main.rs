mod chip8;
use chip8::{Chip8, SPRITES};
use std::{thread, time::Duration};
use raylib::prelude::*;

fn main() {
    let (mut rl, thread) = raylib::init().size(640, 320).title("Chip 8 DEV").build();
    let mut chip8 = Chip8::new();
    chip8.initialize();

    while !rl.window_should_close() {
        // Detect window close button or ESC key
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        // Check State of Hardware

        // Dispatch Next Opcode
        let opcode = chip8.get_next_op();
        println!("Opcode: {:#02x}\n __Registers__", opcode);
        chip8.dispatch(opcode);
        chip8.registers.print_status();

        // Update Keyboard

        // Screen
        for x in 0..64 {
            for y in 0..32 {
                if chip8.get_pixel(x, y) {
                    d.draw_rectangle((x * 10) as i32, (y * 10) as i32, 10, 10, Color::RAYWHITE);
                }
            }
        }

        thread::sleep(Duration::from_millis(1));
    }
}
