use crate::display::Display;
use crate::keypad::Keypad;
use rand::Rng;

pub const PROGRAM_START: u16 = 0x200;

pub struct Cpu {
    pub i: u16,             // index register
    pub pc: u16,            // program counter
    pub memory: [u8; 4096], // 4096 bytes of memory.
    pub v: [u8; 16],        // 16 registers
    pub stack: [u16; 16],
    pub sp: u8, // stack pointer
    pub display: Display,
    pub keypad: Keypad,
    pub dt: u8, // delay timer
    pub st: u8, // sound timer
}

impl Cpu {
    pub fn load_rom(&mut self, rom: &[u8]) {
        for (i, byte) in rom.iter().enumerate() {
            self.memory[i] = *byte;
        }
    }
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            i: 0,
            pc: PROGRAM_START,
            memory: [0; 4096],
            v: [0; 16],
            stack: [0; 16],
            sp: 0,
            display: Display::new(),
            keypad: Keypad::new(),
            dt: 0,
            st: 0,
        }
    }

    fn read_opcode(&self) -> u16 {
        // read a 16 bit word from ram
        let opcode: u16 = (self.memory[self.pc as usize] as u16) << 8
            | (self.memory[self.pc as usize + 1] as u16);
        opcode
    }

    pub fn emulate_cycle(&mut self) {
        // read op code
        let opcode = self.read_opcode();

        self.execute_opcode(opcode);

        if self.dt > 0 {
            self.dt -= 1;
        }
        if self.st > 0 {
            if self.st == 1 {
                println!("BEEP!");
            }
            self.st -= 1;
        }
    }

    fn execute_opcode(&mut self, opcode: u16) {
        // opcode parameters
        let addr = opcode & 0xFFF; // lowest 12 bits
        let byte = (opcode & 0x0FF) as u8; // lowest 8 bits
        let n = opcode & 0x00F; // lowest 4 bits (nibble)
        let x = ((opcode & 0x0F00) >> 8) as usize; // lower 4 bits of the high byte
        let y = ((opcode & 0x00F0) >> 4) as usize; // higher 4 bits of the low byte

        // op_k = k highest bit
        let op_1 = (opcode & 0xF000) >> 12;
        let op_2 = (opcode & 0x0F00) >> 8;
        let op_3 = (opcode & 0x00F0) >> 4;
        let op_4 = opcode & 0x000F;

        self.pc += 2;

        match (op_1, op_2, op_3, op_4) {
            (0x0, 0x0, 0xE, 0x0) => {
                // CLS
                self.display.cls();
            }

            (0x0, 0x0, 0xE, 0xE) => {
                // RET
                self.sp -= 1;
                self.pc = self.stack[self.sp as usize];
            }

            (0x1, _, _, _) => {
                // JP addr
                self.pc = addr;
            }

            (0x2, _, _, _) => {
                // CALL addr

                /*
                   1. increment stack pointer
                   2. put current pc on top of the stack
                   3. set pc to addr
                */
                self.stack[self.sp as usize] = self.pc;
                self.sp += 1;
                self.pc = addr;
            }

            (0x3, _, _, _) => {
                // SE Vx byte
                self.pc += if self.v[x] == byte { 2 } else { 0 };
            }

            (0x4, _, _, _) => {
                // SNE Vx, byte
                self.pc += if self.v[x] != byte { 2 } else { 0 };
            }

            (0x5, _, _, 0x0) => {
                // SE Vx, Vy
                self.pc += if self.v[x] == self.v[y] { 2 } else { 0 };
            }

            (0x6, _, _, _) => {
                // LD Vx, byte
                self.v[x] = byte;
            }

            (0x7, _, _, _) => {
                // ADD Vx, byte
                self.v[x] += byte;
            }

            (0x8, _, _, 0x0) => {
                // LD Vx, Vy
                self.v[x] = self.v[y];
            }

            (0x8, _, _, 0x1) => {
                // OR Vx, Vy
                self.v[x] |= self.v[y];
            }

            (0x8, _, _, 0x2) => {
                // AND Vx, Vy
                self.v[x] &= self.v[y];
            }

            (0x8, _, _, 0x3) => {
                // XOR Vx, Vy
                self.v[x] ^= self.v[y];
            }

            (0x8, _, _, 0x4) => {
                // ADD Vx, Vy
                let (sum, overflow) = self.v[x].overflowing_add(self.v[y]);
                match overflow {
                    true => self.v[0xF] = 1,
                    false => self.v[0xF] = 0,
                }
                self.v[x] = sum;
            }

            (0x8, _, _, 0x5) => {
                // SUB Vx, Vy
                let (diff, overflow) = self.v[x].overflowing_sub(self.v[y]);
                match overflow {
                    true => self.v[0xF] = 0,
                    false => self.v[0xF] = 1,
                }
                self.v[x] = diff;
            }

            (0x8, _, _, 0x6) => {
                // SHR Vx {, Vy}
                if self.v[x] & 1 == 1 {
                    self.v[0xF] = 1;
                } else {
                    self.v[x] /= 2;
                }
            }

            (0x8, _, _, 0x7) => {
                // SUBN Vx, Vy
                let (res, overflow) = self.v[y].overflowing_sub(self.v[x]);
                match overflow {
                    true => self.v[0xF] = 0,
                    false => self.v[0xF] = 1,
                }
                self.v[x] = res;
            }

            (0x8, _, _, 0xE) => {
                // SHL Vx {, Vy}
                self.v[0xF] = self.v[x] & 0x80;
                self.v[x] <<= 1;
            }

            (0x9, _, _, 0x0) => {
                // SNE Vx, Vy
                self.pc += if self.v[x] != self.v[y] { 2 } else { 0 };
            }

            (0xA, _, _, _) => {
                // LD I, addr
                self.i = addr;
            }

            (0xB, _, _, _) => {
                // JP V0, addr
                self.pc = addr + (self.v[0] as u16);
            }

            (0xC, _, _, _) => {
                // RND Vx, byte
                let mut rng = rand::thread_rng();

                self.v[x] = byte & (rng.gen_range(0, 256) as u8);
            }

            (0xD, _, _, _) => {
                // DRW Vx, Vy, nibble
                let collision = self.display.draw(
                    self.v[x] as usize,
                    self.v[y] as usize,
                    &self.memory[self.i as usize..(self.i + n) as u16 as usize],
                );
                self.v[0xF] = if collision { 0 } else { 1 };
            }

            (0xE, _, 0x9, 0xE) => {
                // SKP Vx
                self.pc += if self.keypad.is_pressed(self.v[x]) {
                    2
                } else {
                    0
                };
            }

            (0xE, _, 0xA, 0x1) => {
                // SKNP Vx
                self.pc += if !self.keypad.is_pressed(self.v[x]) {
                    2
                } else {
                    0
                };
            }

            (0xF, _, 0x0, 0x7) => {
                // LD Vx, DT
                self.v[x] = self.dt;
            }

            (0xF, _, 0x0, 0xA) => {
                // LD Vx, K
                self.pc -= 2;
                for (i, key) in self.keypad.keys.iter().enumerate() {
                    if *key {
                        self.v[x] = i as u8;
                        self.pc += 2;
                    }
                }
            }

            (0xF, _, 0x1, 0x5) => {
                // LD DT, Vx
                self.dt = self.v[x];
            }

            (0xF, _, 0x1, 0x8) => {
                // LD ST, Vx
                self.st = self.v[x];
            }

            (0xF, _, 0x1, 0xE) => {
                // ADD I, Vx
                self.i += self.v[x] as u16;
            }

            (0xF, _, 0x2, 0x9) => {
                // LD F, Vx
                self.i = self.v[x] as u16 * 5;
            }

            (0xF, _, 0x3, 0x3) => {
                // LD B, Vx
                self.memory[self.i as usize] = self.v[x] / 100;
                self.memory[self.i as usize + 1] = (self.v[x] / 10) % 10;
                self.memory[self.i as usize + 2] = self.v[x] % 10;
            }

            (0xF, _, 0x5, 0x5) => {
                // LD [I], Vx
                for i in 0..x + 1 {
                    self.memory[self.i as usize + i] = self.v[i];
                }
            }

            (0xF, _, 0x6, 0x5) => {
                // LD Vx, [I]
                for i in 0..x + 1 {
                    self.v[i] = self.memory[self.i as usize + i];
                }
            }

            (_, _, _, _) => (),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Cpu;
    use super::PROGRAM_START;

    #[test]
    fn opcode_jp() {
        let mut cpu = Cpu::new();
        cpu.execute_opcode(0x1A2A);
        assert_eq!(cpu.pc, 0x0A2A, "the program counter is updated");
    }

    #[test]
    fn opcode_call() {
        let mut cpu = Cpu::new();
        let addr = 0x23;
        cpu.pc = addr;

        cpu.execute_opcode(0x2ABC);

        assert_eq!(
            cpu.pc, 0x0ABC,
            "the program counter is updated to the new address"
        );
        assert_eq!(cpu.sp, 1, "the stack pointer is incremented");
        assert_eq!(
            cpu.stack[0],
            addr + 2,
            "the stack stores the previous address"
        );
    }

    #[test]
    fn opcode_se_vx_byte() {
        let mut cpu = Cpu::new();
        cpu.v[1] = 0xFE;

        // vx == byte
        cpu.execute_opcode(0x31FE);
        assert_eq!(cpu.pc, PROGRAM_START + 4, "the stack pointer skips");

        // vx != byte
        cpu.execute_opcode(0x31FA);
        assert_eq!(
            cpu.pc,
            PROGRAM_START + 6,
            "the stack pointer is incremented"
        );
    }

    #[test]
    fn opcode_sne_vx_byte() {
        let mut cpu = Cpu::new();
        cpu.v[1] = 0xFE;

        // vx == byte
        cpu.execute_opcode(0x41FE);
        assert_eq!(
            cpu.pc,
            PROGRAM_START + 2,
            "the stack pointer is incremented"
        );

        // vx != byte
        cpu.execute_opcode(0x41FA);
        assert_eq!(cpu.pc, PROGRAM_START + 6, "the stack pointer skips");
    }

    #[test]
    fn opcode_se_vx_vy() {
        let mut cpu = Cpu::new();
        cpu.v[1] = 1;
        cpu.v[2] = 3;
        cpu.v[3] = 3;

        // vx == vy
        cpu.execute_opcode(0x5230);
        assert_eq!(cpu.pc, PROGRAM_START + 4, "the stack pointer skips");

        // vx != vy
        cpu.execute_opcode(0x5130);
        assert_eq!(
            cpu.pc,
            PROGRAM_START + 6,
            "the stack pointer is incremented"
        );
    }

    #[test]
    fn opcode_sne_vx_vy() {
        let mut cpu = Cpu::new();
        cpu.v[1] = 1;
        cpu.v[2] = 3;
        cpu.v[3] = 3;

        // vx == vy
        cpu.execute_opcode(0x9230);
        assert_eq!(
            cpu.pc,
            PROGRAM_START + 2,
            "the stack pointer is incremented"
        );

        // vx != vy
        cpu.execute_opcode(0x9130);
        assert_eq!(cpu.pc, PROGRAM_START + 6, "the stack pointer skips");
    }

    #[test]
    fn opcode_add_vx_byte() {
        let mut cpu = Cpu::new();
        cpu.v[1] = 3;

        cpu.execute_opcode(0x7101);
        assert_eq!(cpu.v[1], 4, "Vx was incremented by one");
    }

    #[test]
    fn opcode_ld_vx_vy() {
        let mut cpu = Cpu::new();
        cpu.v[1] = 3;
        cpu.v[0] = 0;

        cpu.execute_opcode(0x8010);
        assert_eq!(cpu.v[0], 3, "Vx was loaded with vy");
    }

    #[test]
    fn opcode_or_vx_vy() {
        let mut cpu = Cpu::new();
        cpu.v[2] = 0b01101100;
        cpu.v[3] = 0b11001110;

        cpu.execute_opcode(0x8231);
        assert_eq!(cpu.v[2], 0b11101110, "Vx was loaded with vx OR vy");
    }

    #[test]
    fn opcode_and_vx_vy() {
        let mut cpu = Cpu::new();
        cpu.v[2] = 0b01101100;
        cpu.v[3] = 0b11001110;

        cpu.execute_opcode(0x8232);
        assert_eq!(cpu.v[2], 0b01001100, "Vx was loaded with vx AND vy");
    }

    #[test]
    fn opcode_xor_vx_vy() {
        let mut cpu = Cpu::new();
        cpu.v[2] = 0b01101100;
        cpu.v[3] = 0b11001110;

        cpu.execute_opcode(0x8233);
        assert_eq!(cpu.v[2], 0b10100010, "Vx was loaded with vx XOR vy");
    }

    #[test]
    fn opcode_add_vx_vy() {
        let mut cpu = Cpu::new();
        cpu.v[1] = 10;
        cpu.v[2] = 100;
        cpu.v[3] = 250;

        cpu.execute_opcode(0x8124);
        assert_eq!(cpu.v[1], 110, "Vx was loaded with vx + vy");
        assert_eq!(cpu.v[0xF], 0, "no overflow occured");

        cpu.execute_opcode(0x8134);
        assert_eq!(cpu.v[1], 0x68, "Vx was loaded with vx + vy");
        assert_eq!(cpu.v[0xF], 1, "overflow occured");
    }

    #[test]
    fn opcode_ld_i_vx() {
        let mut cpu = Cpu::new();
        cpu.v[0] = 5;
        cpu.v[1] = 4;
        cpu.v[2] = 3;
        cpu.v[3] = 2;
        cpu.i = 0x300;

        // load v0 - v2 into memory at i
        cpu.execute_opcode(0xF255);
        assert_eq!(
            cpu.memory[cpu.i as usize], 5,
            "V0 was loaded into memory at i"
        );
        assert_eq!(
            cpu.memory[cpu.i as usize + 1],
            4,
            "V1 was loaded into memory at i + 1"
        );
        assert_eq!(
            cpu.memory[cpu.i as usize + 2],
            3,
            "V2 was loaded into memory at i + 2"
        );
        assert_eq!(cpu.memory[cpu.i as usize + 3], 0, "i + 3 was not loaded");
    }

    #[test]
    fn opcode_ld_b_vx() {
        let mut cpu = Cpu::new();
        cpu.i = 0x300;
        cpu.v[2] = 234;

        // load v0 - v2 from memory at i
        cpu.execute_opcode(0xF233);
        assert_eq!(cpu.memory[cpu.i as usize], 2, "hundreds");
        assert_eq!(cpu.memory[cpu.i as usize + 1], 3, "tens");
        assert_eq!(cpu.memory[cpu.i as usize + 2], 4, "digits");
    }

    #[test]
    fn opcode_ld_vx_i() {
        let mut cpu = Cpu::new();
        cpu.i = 0x300;
        cpu.memory[cpu.i as usize] = 5;
        cpu.memory[cpu.i as usize + 1] = 4;
        cpu.memory[cpu.i as usize + 2] = 3;
        cpu.memory[cpu.i as usize + 3] = 2;

        // load v0 - v2 from memory at i
        cpu.execute_opcode(0xF265);
        assert_eq!(cpu.v[0], 5, "V0 was loaded from memory at i");
        assert_eq!(cpu.v[1], 4, "V1 was loaded from memory at i + 1");
        assert_eq!(cpu.v[2], 3, "V2 was loaded from memory at i + 2");
        assert_eq!(cpu.v[3], 0, "i + 3 was not loaded");
    }

    #[test]
    fn opcode_ret() {
        let mut cpu = Cpu::new();
        let addr = 0x23;
        cpu.pc = addr;

        // jump to 0x0ABC
        cpu.execute_opcode(0x2ABC);
        // return
        cpu.execute_opcode(0x00EE);

        assert_eq!(
            cpu.pc, 0x25,
            "the program counter is updated to the new address"
        );
        assert_eq!(cpu.sp, 0, "the stack pointer is decremented");
    }

    #[test]
    fn opcode_ld_i_addr() {
        let mut cpu = Cpu::new();

        cpu.execute_opcode(0x61AA);
        assert_eq!(cpu.v[1], 0xAA, "V1 is set");
        assert_eq!(
            cpu.pc,
            PROGRAM_START + 2,
            "the program counter is advanced two bytes"
        );

        cpu.execute_opcode(0x621A);
        assert_eq!(cpu.v[2], 0x1A, "V2 is set");
        assert_eq!(
            cpu.pc,
            PROGRAM_START + 4,
            "the program counter is advanced two bytes"
        );

        cpu.execute_opcode(0x6A15);
        assert_eq!(cpu.v[10], 0x15, "V10 is set");
        assert_eq!(
            cpu.pc,
            PROGRAM_START + 6,
            "the program counter is advanced two bytes"
        );
    }

    #[test]
    fn opcode_axxx() {
        let mut cpu = Cpu::new();
        cpu.execute_opcode(0xAFAF);

        assert_eq!(cpu.i, 0x0FAF, "the 'i' register is updated");
        assert_eq!(
            cpu.pc,
            PROGRAM_START + 2,
            "the program counter is advanced two bytes"
        );
    }
}
