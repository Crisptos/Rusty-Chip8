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

// A Struct that contains every Chip 8 register
pub struct Registers {
    pub v: [u8; 16], // Multipurpose 1 byte V registers
    pub i: u16,      // 2 Byte I Register
    pub dt: u8,      // Delay Timer
    pub st: u8,      // Sound Timer
    pub pc: u16,     // Program Counter
    pub sp: u8,      // Stack Pointer
}

impl Registers {
    // Register Constructor
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

// A Struct that contains all of the virtual hardware that composes the Chip 8
pub struct Chip8 {
    pub memory: [u8; 4096],   // Chip 8 RAM
    pub registers: Registers, // Registers
    pub stack: Vec<u16>,      // Call Stack
    pub screen: [bool; 2048], // 64x32 Monochromatic Screen Represented as Bools (Pixel on or off)
    pub keyboard: [bool; 16], // An Array of all 16 Possible Keys 1 - F
}

impl Chip8 {
    // Constructor to Initialize Hardware
    pub fn new() -> Chip8 {
        Chip8 {
            memory: [0; 4096],
            registers: Registers::new(),
            stack: vec![0; 16],
            screen: [false; 2048],
            keyboard: [false; 16],
        }
    }

    // Initialize the system
    pub fn initialize(&mut self) {
        // Load the default sprites into address 0x00
        self.memory[0x00..SPRITES.len()].copy_from_slice(&SPRITES);
        let rom: Vec<u8> = load_rom();
        if rom.len() == 1 {
            return;
        }

        // Copy the rom memory into RAM starting at 0x200
        self.memory[0x200..0x200 + rom.len()].copy_from_slice(&rom);
        println!("Chip 8 ROM Loaded...");

        // Set entry point
        self.registers.pc = 0x200;
    }

    // Get pixel's on or off status at index x,y
    pub fn get_pixel(&mut self, x: usize, y: usize) -> bool {
        self.screen[x + (64 * y)]
    }

    // Draw a sprite to the screen at x,y located at the memory index
    pub fn draw(&mut self, x: usize, y: usize, memory_index: usize, size: usize) -> bool {
        let mut is_colliding: bool = false;
        for y_offset in 0..size {
            // We get the needed byte at the current Y-Index to get which bits are 1
            let y_index = self.memory[memory_index + y_offset];
            for x_offset in 0..8 {
                if (y_index & (0x80 >> x_offset)) != 0 {
                    self.screen[((x + x_offset) % 64) + (64 * ((y + y_offset) % 32))] ^= true;
                    is_colliding = true;
                }
            }
        }

        is_colliding
    }

    // Get the next opcode from memory
    pub fn get_next_op(&mut self) -> u16 {
        let index: usize = self.registers.pc as usize;
        let mut opcode: u16;

        opcode = self.memory[index] as u16;
        opcode <<= 8;
        opcode |= self.memory[index + 1] as u16;
        opcode
    }

    // Handle opcodes with a switch case
    pub fn dispatch(&mut self, opcode: u16) {
        // Opcodes that don't provide args
        match opcode {
            0x00E0 => {
                // CLS (Clear Screen)
                self.screen = [false; 2048];
            }

            0x00EE => {
                // RET (Return)
                self.registers.pc = match self.stack.pop() {
                    Some(adr) => adr,
                    None => {
                        println!("CRITICAL STACK ERROR, POPPED NO ADDRESS DURING RET...");
                        0
                    }
                };
                self.registers.sp -= 1;
            }
            _ => {}
        };

        // Used for Opcodes that provide 1 arg. _NNN & 0FFF = NNN
        let nnn: u16 = opcode & 0x0FFF;
        match opcode & 0xF000 {
            0x1000 => {
                // JP (Jump)
                self.registers.pc = nnn;
            }

            0x2000 => {
                // CALL (Call Subroutine)
                self.registers.sp += 1;
                self.stack.push(self.registers.pc);
                self.registers.pc = nnn;
            }

            0xA000 => {
                // LD I (Load I)
                self.registers.i = nnn;
            }

            0xB000 => {
                // JP V0 (Jump nnn + V0)
                self.registers.pc = nnn + (self.registers.v[0] as u16);
            }

            _ => {}
        };

        // Used for Opcodes that provide 2 arg. _xkk & 0F00 = x  _xkk & 00FF = kk
        let x: u8 = ((opcode & 0x0F00) >> 8) as u8;
        let y: u8 = ((opcode & 0x00F0) >> 4) as u8;
        let kk: u8 = (opcode & 0x00FF) as u8;
        match opcode & 0xF000 {
            0x3000 => {
                // SE VX KK (Skip Equals VX KK)
                if self.registers.v[x as usize] == kk {
                    self.registers.pc += 2;
                }
            }

            0x4000 => {
                // SNE VX KK (Skip Not Equals VX KK)
                if self.registers.v[x as usize] != kk {
                    self.registers.pc += 2;
                }
            }

            0x5000 => {
                // SE VX VY (Skip Equals VX VY)
                if self.registers.v[x as usize] == self.registers.v[y as usize] {
                    self.registers.pc += 2;
                }
            }

            0x6000 => {
                // LD VX KK (Load VX KK)
                self.registers.v[x as usize] = kk;
            }

            0x7000 => {
                // ADD VX KK (Add VX KK)
                self.registers.v[x as usize] += kk;
            }

            0x9000 => {
                // SNE VX KK (Skip Not Equal VX VY)
                if self.registers.v[x as usize] != self.registers.v[y as usize] {
                    self.registers.pc += 0x02;
                }
            }

            _ => {}
        }

        match opcode & 0xF00F {
            0x8000 => {
                // LD VX VY (Load VX VY)
                self.registers.v[x as usize] = self.registers.v[y as usize];
            }

            0x8001 => {
                // OR VX VY (Or VX VY)
                self.registers.v[x as usize] |= self.registers.v[y as usize];
            }

            0x8002 => {
                // AND VX VY (And VX VY)
                self.registers.v[x as usize] &= self.registers.v[y as usize];
            }

            0x8003 => {
                // XOR VX VY (Xor VX VY)
                self.registers.v[x as usize] ^= self.registers.v[y as usize];
            }

            0x8004 => {
                // ADD VX VY (Add VX VY)
                self.registers.v[0x0F] = 0x00;
                let temp =
                    ((self.registers.v[x as usize] as u16) + (self.registers.v[y as usize] as u16));
                println!("{:#02x}", temp);
                if temp & 0xFF00 != 0 {
                    self.registers.v[0x0F] = 0x01;
                }
                self.registers.v[x as usize] = (temp & 0x00FF) as u8;
            }

            0x8005 => {
                // SUB VX VY (Sub VX VY)
                self.registers.v[0x0F] = 0x00;
                if self.registers.v[x as usize] > self.registers.v[y as usize] {
                    self.registers.v[0x0F] = 0x01;
                }
                self.registers.v[x as usize] =
                    self.registers.v[x as usize].wrapping_sub(self.registers.v[y as usize]);
            }

            0x8006 => {
                // SHR VX (Shift Right VX)
                self.registers.v[0x0F] = if self.registers.v[x as usize] & 0x01 == 1 {
                    0x01
                } else {
                    0x00
                };

                self.registers.v[x as usize] /= 2;
            }

            0x8007 => {
                // SUBN VX VY (Subtract VX VY)
                self.registers.v[0x0F] =
                    if self.registers.v[y as usize] > self.registers.v[x as usize] {
                        0x01
                    } else {
                        0x00
                    };

                self.registers.v[x as usize] =
                    self.registers.v[y as usize].wrapping_sub(self.registers.v[x as usize]);
            }

            0x800E => {
                // SHL VX (Shift Left VX)
                self.registers.v[0x0F] = if self.registers.v[x as usize] >> 7 == 1 {
                    0x01
                } else {
                    0x00
                };

                self.registers.v[x as usize] = self.registers.v[x as usize].wrapping_mul(2);
            }

            _ => {}
        }
    }
}

// Load file and return it as a Vec of bytes
fn load_rom() -> Vec<u8> {
    match std::fs::read("roms/logo.ch8") {
        Ok(r) => r,
        Err(e) => {
            println!("{}", e);
            vec![0]
        }
    }
}
