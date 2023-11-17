# Rusty-Chip8

### Dependencies
- Raylib
- Rand

### History
For quite some time it's been a strong interest of mine to dive into the development of emulation systems and interpreters. This project isn't my first implementation of the Chip 8 but, it's the one that I'm definitely taking the most serious and I aim to have it be as close of an emulation as it can.

### Goals
The goal of this project is to create an interpreter that can be run on the desktop and in browser when compiled to WASM.

### Tasks
- [X] Create main emulation loop
- [X] Implement virtual hardware (Screen, Keyboard, RAM, Registers)
- [X] Implement drawing to the screen through memory
- [X] Load a chip 8 rom and copy the binary into memory
- [X] Iterate through the loaded program
- [X] Dispatch opcodes (not all opcodes have been implemented yet... but soon :D)   
- [X] Add in a beep sound
- [ ] Touch up the way the delay timer functions
- [ ] Refactor the check for the controls
- [ ] Test more ROMs 

### Emulator Journey Checklist
This is one of my first projects in Rust and so far the language has been a joy to use. It'll most likely be my choice for future emulator development.
Here's what my current roadmap for emulator develpment plans looks like...
#### Chip8 -> Gameboy (Z80 Emulation) -> NES (6502 Emulation) -> TBD
