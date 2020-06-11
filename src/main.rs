mod chip8;

fn main() {
    let disp = chip8::Display {
        d: true
    };
    // initialize chip8
    let mut CPU = chip8::Cpu {
        I: 0,
        pc: 0x200,
        memory: [0; 4096],
        v: [0; 16],
        stack: [0;16],
        sp: 0,
        display: disp,
        dt: 0,
        st: 0,
    };

    loop {
        CPU.emulate_cycle();    // emulate one cycle
    }
}
