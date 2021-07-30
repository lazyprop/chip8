mod display;
mod cpu;
mod keypad;

use display::Display;
use cpu::Cpu;
use keypad::Keypad;

fn main() {
    let mut CPU = Cpu::new();

    loop {
        CPU.emulate_cycle();    // emulate one cycle
    }
}
