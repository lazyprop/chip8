pub struct Cpu {
    pub I: u16,     // index register
    pub pc: u16,    // program counter
    pub memory: [u8; 4096],     // 4096 bytes of memory.
    pub v: [u8; 16],  // 16 registers
    pub stack: [u16; 16],
    pub sp: u8, // stack pointer
    pub display: Display,
}

impl Cpu {
    pub fn emulate_cycle(&mut self) {
        // read op code
        let opcode = read_opcode(self.memory, self.pc);

        // match opcode 
        // execute opcode
        self.execute_opcode(opcode);

        // update counters
    }

    fn execute_opcode (&mut self, opcode: u16) {
        // opcode parameters
        let addr = opcode & 0xFFF; // lowest 12 bits
        let kk = (opcode & 0x0FF) as u8;   // lowset 8 bits
        let n = opcode & 0x00F;    // lowest 4 bits
        let x = ((opcode >> 8) & 0xF0) as usize;    // lower 4 bits of the high byte
        let y = ((opcode >> 4) & 0x0F) as usize;    // higher 4 bits of the low byte
        
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
            },

            (0x0, 0x0, 0xE, 0xE) => {
                // RET
                self.sp -= 1;
                self.pc = self.stack[self.sp as usize];
            },

            (0x1, _, _, _) => {
                // JP addr
                self.pc = addr;
            },

            (0x2, _, _, _) => {
                // CALL addr

                /*
                   1. increment stack pointer
                   2. put current pc on top of the stack
                   3. set pc to addr
                */
                self.pc = addr;
            },

            (0x3, _, _, _) => {
                // SE Vx byte
                self.pc += if self.v[x] == kk { 2 } else { 0 };
            },

            (0x4, _, _, _) => {
                self.pc+= if self.v[x] != kk { 2 } else { 0 };
            }

            (0x5, _, _, 0x0) => {
                // SE Vx, Vy 
                self.pc += if self.v[x] == self.v[y] { 2 } else { 0 };
            }, 

            (0x6, _, _, _) => {
                // LD Vx, byte
                self.v[x] = kk;
            },

            (0x7, _, _, _) => {
                // ADD Vx, byte
                self.v[x] += kk;
            },

            (0x8, _, _, 0x0) => {
                // LD Vx, Vy
                self.v[x] = self.v[y];
            },

            (0x8, _, _, 0x1) => {
                // OR Vx, Vy
                self.v[x] = self.v[x] | self.v[y];
            },

            (0x8, _, _, 0x2) => {
                // AND Vx, Vy
                self.v[x] = self.v[x] & self.v[y];
            },

            (0x8, _, _, 0x3) => {
                // XOR Vx, Vy
                self.v[x] = self.v[x] ^ self.v[y];
            },

            (0x8, _, _, 0x4) => {
                // ADD Vx, Vy
                let (sum, overflow) = self.v[x].overflowing_add(self.v[y]);
                match overflow {
                    true => self.v[0xF] = 1,
                    false => self.v[0xF] = 0,
                }
                self.v[x] = sum;
            },

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
                if self.v[x] & 1 == 1{
                    self.v[0xF] = 1;
                } else {
                    self.v[x] /= 2;
                }
            }, 

            (0x8, _, _, 0x7) => {
                // SUBN Vx, Vy
                let (res, overflow) = self.v[y].overflowing_sub(self.v[x]);
                match overflow {
                    true => self.v[0xF] = 0,
                    false => self.v[0xF] = 1,
                }
                self.v[x] = res;
            },

            (0x8, _, _, 0xE) => {
                // SHL Vx {, Vy}
                self.v[0xF] = self.v[x] & 0x80;
                self.v[x] <<= 1;
            },

            (_, _, _, _) => {
                // temporary
            }
        }
    }
}

fn read_opcode(memory: [u8; 4096], pc: u16) -> u16 {
    // read a 16bit word from ram
    let opcode: u16 = (memory[pc as usize] as u16) << 8 | 
        (memory[pc as usize + 1] as u16);
    opcode
}

pub struct Display {
    pub d:  bool,
}

impl Display {
    pub fn cls(&mut self) {
        self.d = false;
    }
}
