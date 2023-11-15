mod chip8;
use chip8::{Chip8, SPRITES};
use raylib::prelude::*;

fn main() {
    let (mut rl, thread) = raylib::init().size(640, 320).title("Chip 8 DEV").build();
    let mut chip8 = Chip8::new();
    chip8.initialize();

    chip8.registers.v[0x01] = 0x20;
    chip8.dispatch(0x7140);
    println!("{:#02x}", chip8.registers.v[0x01]);

    while !rl.window_should_close() {
        // Detect window close button or ESC key
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        // Check State of Hardware

        // Update Keyboard

        // Screen
        for x in 0..64 {
            for y in 0..32 {
                if chip8.get_pixel(x, y) {
                    d.draw_rectangle((x * 10) as i32, (y * 10) as i32, 10, 10, Color::RAYWHITE);
                }
            }
        }
    }
}
