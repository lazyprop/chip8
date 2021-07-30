mod display;
mod cpu;
mod keypad;

use display::Display;
use cpu::Cpu;
use keypad::Keypad;

fn main() {
    let display = Display {
        d: true
    };
    let keypad = Keypad {
        keys: [false; 16]
    };

    // initialize chip8
    let mut CPU = Cpu {
        I: 0,
        pc: 0x200,
        memory: [0; 4096],
        v: [0; 16],
        stack: [0;16],
        sp: 0,
        display,
        keypad,
        dt: 0,
        st: 0,
    };

    loop {
        CPU.emulate_cycle();    // emulate one cycle
    }
}
