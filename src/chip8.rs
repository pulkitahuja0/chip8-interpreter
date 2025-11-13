use rand::Rng;
use rand::rngs::ThreadRng;

use crate::registers::Registers;
use crate::stack::Stack;

const MEMORY_SIZE: usize = 4096;

pub struct Chip8 {
    memory: [u8; MEMORY_SIZE],
    register: Registers,
    stack: Stack,
    pc: u16,
    rng: ThreadRng,
}

fn panic_on_opcode(opcode: u16, pc: u16) -> String {
    format!("Bad opcode {:#04X} at address {:#05X}", opcode, pc)
}

fn sub_error(opcode: u16, pc: u16, error: &str) -> String {
    format!("{} at opcode {:#04X} at address {:#05X}", error, opcode, pc)
}

// Functions relating to bit operations
fn create_nnn(b: u16, c: u16, d: u16) -> u16 {
    ((b) << 8) | ((c) << 4) | (d)
}

fn create_nn(c: u16, d: u16) -> u16 {
    (c << 4) | d
}

// TODO: Error handling .set for bad registers
impl Chip8 {
    pub fn new(rom: &[u8]) -> Self {
        let mut memory: [u8; MEMORY_SIZE] = [0; MEMORY_SIZE];

        // TODO: load fontset here

        // TODO: Handle bad ROM
        if 0x200 + rom.len() > MEMORY_SIZE {
            panic!("Bad ROM detected");
        }

        memory[0x200..(0x200 + rom.len())].copy_from_slice(rom);

        Self {
            memory,
            register: Registers::new(),
            stack: Stack::new(),
            pc: 0x200,
            rng: rand::rng(),
        }
    }

    pub fn step(&mut self) -> Result<(), String> {
        let opcode = ((self.memory[self.pc as usize] as u16) << 8)
            | (self.memory[(self.pc + 1) as usize] as u16);
        let pc = self.pc; // Address of current instruction

        self.pc += 2; // Address of next instruction (use for stack)

        let a = (opcode & 0xF000) >> 12;
        let b = (opcode & 0x0F00) >> 8;
        let c = (opcode & 0x00F0) >> 4;
        let d = opcode & 0x000F;

        match a {
            0 => {
                if b == 0 && c == 0xE {
                    if d == 0 {
                        // 00E0
                        // TODO: Clear screen
                        return Ok(());
                    } else if d == 0xE {
                        // 00EE
                        self.pc = self.stack.return_subroutine();
                        return Ok(());
                    } else {
                        return Err(panic_on_opcode(opcode, pc));
                    }
                } else {
                    return Err(panic_on_opcode(opcode, pc));
                }
            }
            1 => {
                // 1NNN
                let nnn = create_nnn(b, c, d);
                if (nnn as usize) < MEMORY_SIZE {
                    self.pc = nnn;
                    return Ok(());
                } else {
                    return Err(panic_on_opcode(opcode, pc));
                }
            }
            2 => {
                // 2NNN
                let nnn = create_nnn(b, c, d);
                if (nnn as usize) < MEMORY_SIZE {
                    self.stack.subroutine(self.pc);
                    self.pc = nnn;
                    return Ok(());
                } else {
                    return Err(panic_on_opcode(opcode, pc));
                }
            }
            3 => {
                // 3XNN
                let nn = create_nn(c, d);
                {
                    let v = self.register.get_v(b as u8);
                    match v {
                        Ok(value) => {
                            if value == nn as u8 {
                                self.pc += 2;
                            }

                            return Ok(());
                        }
                        Err(err) => {
                            return Err(sub_error(opcode, pc, err));
                        }
                    }
                }
            }
            4 => {
                // 4XNN
                let nn = create_nn(c, d);
                {
                    let v = self.register.get_v(b as u8);
                    match v {
                        Ok(value) => {
                            if value != nn as u8 {
                                self.pc += 2;
                            }

                            return Ok(());
                        }
                        Err(err) => {
                            return Err(sub_error(opcode, pc, err));
                        }
                    }
                }
            }
            5 => {
                // 5XY0
                if d == 0 {
                    if self.register.get_v(b as u8) == self.register.get_v(c as u8) {
                        self.pc += 2;
                    }
                    return Ok(());
                } else {
                    return Err(panic_on_opcode(opcode, pc));
                }
            }
            6 => {
                // 6XNN
                let nn = create_nn(c, d);
                match self.register.set_v(b as u8, nn as u8) {
                    Ok(()) => {
                        return Ok(());
                    }
                    Err(err) => {
                        return Err(sub_error(opcode, pc, err));
                    }
                }
            }
            7 => {
                // 7XNN
                let nn = create_nn(c, d);
                match self.register.set_v(b as u8, nn as u8) {
                    Ok(()) => {
                        return Ok(());
                    }
                    Err(err) => {
                        return Err(sub_error(opcode, pc, err));
                    }
                }
            }
            8 => match d {
                0 => {
                    // 8XY0
                    let vy = match self.register.get_v(c as u8) {
                        Ok(value) => value,
                        Err(err) => {
                            return Err(sub_error(opcode, pc, err));
                        }
                    };
                    match self.register.set_v(b as u8, vy) {
                        Ok(()) => {
                            return Ok(());
                        }
                        Err(err) => {
                            return Err(sub_error(opcode, pc, err));
                        }
                    }
                }
                1 => {
                    // 8XY1
                    let vx = match self.register.get_v(b as u8) {
                        Ok(value) => value,
                        Err(err) => {
                            return Err(sub_error(opcode, pc, err));
                        }
                    };
                    let vy = match self.register.get_v(c as u8) {
                        Ok(value) => value,
                        Err(err) => {
                            return Err(sub_error(opcode, pc, err));
                        }
                    };
                    match self.register.set_v(b as u8, vx | vy) {
                        Ok(()) => {
                            return Ok(());
                        }
                        Err(err) => {
                            return Err(sub_error(opcode, pc, err));
                        }
                    }
                }
                2 => {
                    // 8XY2
                    let vx = match self.register.get_v(b as u8) {
                        Ok(value) => value,
                        Err(err) => {
                            return Err(sub_error(opcode, pc, err));
                        }
                    };
                    let vy = match self.register.get_v(c as u8) {
                        Ok(value) => value,
                        Err(err) => {
                            return Err(sub_error(opcode, pc, err));
                        }
                    };
                    match self.register.set_v(b as u8, vx & vy) {
                        Ok(()) => {
                            return Ok(());
                        }
                        Err(err) => {
                            return Err(sub_error(opcode, pc, err));
                        }
                    }
                }
                3 => {
                    // 8XY3
                    let x = match self.register.get_v(b as u8) {
                        Ok(value) => value,
                        Err(err) => {
                            return Err(sub_error(opcode, pc, err));
                        }
                    };
                    let y = match self.register.get_v(c as u8) {
                        Ok(value) => value,
                        Err(err) => {
                            return Err(sub_error(opcode, pc, err));
                        }
                    };
                    match self.register.set_v(b as u8, x ^ y) {
                        Ok(()) => {
                            return Ok(());
                        }
                        Err(err) => {
                            return Err(sub_error(opcode, pc, err));
                        }
                    };
                }
                4 => {
                    // 8XY4
                    let vx = match self.register.get_v(b as u8) {
                        Ok(value) => value,
                        Err(err) => {
                            return Err(sub_error(opcode, pc, err));
                        }
                    };
                    let vy = match self.register.get_v(c as u8) {
                        Ok(value) => value,
                        Err(err) => {
                            return Err(sub_error(opcode, pc, err));
                        }
                    };

                    // Set flag register based on overflow
                    if (vx as u16) + (vy as u16) > 0xFF {
                        match self.register.set_v(0xF, 1) {
                            Ok(()) => {}
                            Err(err) => {
                                return Err(sub_error(opcode, pc, err));
                            }
                        }
                    } else {
                        match self.register.set_v(0xF, 0) {
                            Ok(()) => {}
                            Err(err) => {
                                return Err(sub_error(opcode, pc, err));
                            }
                        }
                    }

                    match self.register.set_v(b as u8, vx.wrapping_add(vy)) {
                        Ok(()) => {
                            return Ok(());
                        }
                        Err(err) => {
                            return Err(sub_error(opcode, pc, err));
                        }
                    }
                }
                5 => {
                    // 8XY5
                    // VX = VX - VY
                    let vx = match self.register.get_v(b as u8) {
                        Ok(value) => value,
                        Err(err) => {
                            return Err(sub_error(opcode, pc, err));
                        }
                    };
                    let vy = match self.register.get_v(c as u8) {
                        Ok(value) => value,
                        Err(err) => {
                            return Err(sub_error(opcode, pc, err));
                        }
                    };

                    // Set flag register based on first operand being larger than second
                    if vx > vy {
                        match self.register.set_v(0xF, 1) {
                            Ok(()) => {}
                            Err(err) => {
                                return Err(sub_error(opcode, pc, err));
                            }
                        };
                    } else {
                        match self.register.set_v(0xF, 0) {
                            Ok(()) => {}
                            Err(err) => {
                                return Err(sub_error(opcode, pc, err));
                            }
                        };
                    }

                    match self.register.set_v(b as u8, vx - vy) {
                        Ok(()) => {
                            return Ok(());
                        }
                        Err(err) => {
                            return Err(sub_error(opcode, pc, err));
                        }
                    }
                }
                6 => {
                    // TODO: Shift (need to add configuration)
                    return Ok(());
                }
                7 => {
                    // 8XY7
                    // VX = VY - VX
                    let vx = match self.register.get_v(b as u8) {
                        Ok(value) => value,
                        Err(err) => {
                            return Err(sub_error(opcode, pc, err));
                        }
                    };
                    let vy = match self.register.get_v(c as u8) {
                        Ok(value) => value,
                        Err(err) => {
                            return Err(sub_error(opcode, pc, err));
                        }
                    };

                    // Set flag register based on first operand being larger than second
                    if vy > vx {
                        match self.register.set_v(0xF, 1) {
                            Ok(()) => {}
                            Err(err) => {
                                return Err(sub_error(opcode, pc, err));
                            }
                        }
                    } else {
                        match self.register.set_v(0xF, 0) {
                            Ok(()) => {}
                            Err(err) => {
                                return Err(sub_error(opcode, pc, err));
                            }
                        }
                    }

                    match self.register.set_v(b as u8, vy - vx) {
                        Ok(()) => {
                            return Ok(());
                        }
                        Err(err) => {
                            return Err(sub_error(opcode, pc, err));
                        }
                    }
                }
                0xE => {
                    // TODO: Shift (need to add configuration)
                    return Ok(());
                }
                _ => {
                    return Err(panic_on_opcode(opcode, pc));
                }
            },
            9 => {
                if d == 0 {
                    let vx = match self.register.get_v(b as u8) {
                        Ok(value) => value,
                        Err(err) => {
                            return Err(sub_error(opcode, pc, err));
                        }
                    };
                    let vy = match self.register.get_v(c as u8) {
                        Ok(value) => value,
                        Err(err) => {
                            return Err(sub_error(opcode, pc, err));
                        }
                    };
                    if vx != vy {
                        self.pc += 2;
                    }
                    return Ok(());
                } else {
                    return Err(panic_on_opcode(opcode, pc));
                }
            }
            0xA => {
                // ANNN
                let nnn = create_nnn(b, c, d);
                self.register.set_index_register(nnn);

                return Ok(());
            }
            0xB => {
                // BNNN
                let nnn = create_nnn(b, c, d);
                let v0 = match self.register.get_v(0) {
                    Ok(value) => value,
                    Err(err) => {
                        return Err(sub_error(opcode, pc, err));
                    }
                };

                if (((nnn as u8) + v0) as usize) < MEMORY_SIZE {
                    self.pc = nnn + (v0 as u16);
                    return Ok(());
                } else {
                    return Err(panic_on_opcode(opcode, pc));
                }

                // TODO: Add BXNN based off of config
            }
            0xC => {
                // CXNN
                let rand_val: u8 = self.rng.random();

                let nn = create_nn(c, d);
                let val = rand_val & (nn as u8);

                match self.register.set_v(b as u8, val) {
                    Ok(()) => {
                        return Ok(());
                    }
                    Err(err) => {
                        return Err(sub_error(opcode, pc, err));
                    }
                }
            }
            0xD => {
                // TODO: Display
                return Ok(());
            }
            0xE => {
                // TODO: Input
                return Ok(());
            }
            0xF => match c {
                1 => {
                    if d == 0xE {
                        // FX1E
                        // TODO: config for ambiguous overflow behavior here
                        self.register
                            .set_index_register(self.register.get_index() + b);
                        return Ok(());
                    } else {
                        return Err(panic_on_opcode(opcode, pc));
                    }
                }
                2 => {
                    if d == 9 {
                        // FX29
                        // TODO: Fonts
                        return Ok(());
                    } else {
                        return Err(panic_on_opcode(opcode, pc));
                    }
                }
                3 => {
                    // FX33
                    // TODO: Binary code to decimal conversion
                    return Ok(());
                }
                5 => {
                    if d == 5 {
                        // FX55
                        if b > 15 {
                            return Err(panic_on_opcode(opcode, pc));
                        }

                        for j in 0..=b {
                            self.memory[(self.register.get_index() + j) as usize] =
                                match self.register.get_v(j as u8) {
                                    Ok(value) => value,
                                    Err(err) => {
                                        return Err(sub_error(opcode, pc, err));
                                    }
                                };
                        }
                        return Ok(());
                    } else {
                        return Err(panic_on_opcode(opcode, pc));
                    }
                }
                6 => {
                    // TODO: Load from memory
                    if d == 5 {
                        // FX65
                        for j in 0..=b {
                            match self.register.set_v(
                                j as u8,
                                self.memory[(self.register.get_index() + j) as usize],
                            ) {
                                Ok(()) => {}
                                Err(err) => {
                                    return Err(sub_error(opcode, pc, err));
                                }
                            }
                        }

                        return Ok(());
                    } else {
                        return Err(panic_on_opcode(opcode, pc));
                    }
                }
                _ => {
                    return Err(panic_on_opcode(opcode, pc));
                }
            },
            _ => {
                return Err(panic_on_opcode(opcode, pc));
            }
        }
    }
}
