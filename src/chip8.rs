pub struct Cpu {
    pub I: u16,     // index register
    pub pc: u16,    // program counter
    pub memory: [u8; 4096],     // 4096 bytes of memory.
    pub reg: [u8; 16],  // 16 registers
    pub opcode: u16,    // current opcode 
}

impl Cpu {
    pub fn emulate_cycle(&mut self) {
        // read op code
        self.opcode = read_opcode(self.memory, self.pc);

        // match opcode 
        // execute opcode
        self.execute_opcode();

        // update counters
    }

    fn execute_opcode (&mut self) {
        // match opcode to instruction
        match self.opcode {
            0xA000 => { // sets I to index NNN
                // execute corresponding instruction
            }, 

            _ => (),
        }
    }
}

fn read_opcode(memory: [u8; 4096], pc: u16) -> u16 {
    // read a 16bit word from ram
    let opcode: u16 = (memory[pc as usize] as u16) | 
        (memory[pc as usize + 1] as u16);
    opcode
}
