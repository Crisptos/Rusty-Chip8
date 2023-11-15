// Hexadecimal Representation of the Standard Sprites
pub const SPRITES: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

pub struct Registers {
    pub v: [u8; 16],
    pub i: u16,
    pub dt: u8,
    pub st: u8,
    pub pc: u16,
    pub sp: u8,
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
            v: [0; 16],
            i: (0),
            dt: (0),
            st: (0),
            pc: (0),
            sp: (0),
        }
    }
}

pub struct Chip8 {
    pub memory: [u8; 4096],
    pub registers: Registers,
    pub stack: Vec<u16>,
    pub screen: [bool; 2048],
    pub keyboard: [bool; 16],
}

impl Chip8 {
    pub fn new() -> Chip8 {
        Chip8 {
            memory: [0; 4096],
            registers: Registers::new(),
            stack: vec![0; 16],
            screen: [false; 2048],
            keyboard: [false; 16],
        }
    }

    pub fn initialize(&mut self) {
        // Load the default sprites into address 0x00
        self.memory[0x00..SPRITES.len()].copy_from_slice(&SPRITES);
        let rom: Vec<u8> = load_rom();
        if rom.len() == 1 {
            return;
        }

        self.memory[0x1FF..0x1FF + rom.len()].copy_from_slice(&rom);
        println!("Chip 8 ROM Loaded...");

        // Set entry point
        self.registers.pc = 0x200;
    }

    pub fn get_pixel(&mut self, x: usize, y: usize) -> bool {
        self.screen[x + (64 * y)]
    }

    pub fn draw(&mut self, x: usize, y: usize, memory_index: usize, size: usize) -> bool {
        let mut is_colliding: bool = false;
        for y_offset in 0..size {
            // We get the needed byte at the current Y-Index to get which bits are 1
            let y_index = self.memory[memory_index + y_offset];
            for x_offset in 0..8 {
                if (y_index & (0x80 >> x_offset)) != 0 {
                    self.screen[((x + x_offset)%64) + (64 * ((y + y_offset)%32))] ^= true;
                    is_colliding = true;
                }
            }
        }

        is_colliding
    }
}

fn load_rom() -> Vec<u8> {
    match std::fs::read("roms/logo.ch8") {
        Ok(r) => r,
        Err(e) => {
            println!("{}", e);
            vec![0]
        }
    }
}
