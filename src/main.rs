mod chip8;

fn main() {
    // initialize chip8
    let mut CPU = chip8::Cpu {
        I: 0,
        pc: 0x200,
        memory: [0; 4096],
        reg: [0; 16],
        opcode: 0,
    };

    CPU.emulate_cycle();
}
