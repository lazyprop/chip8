mod cpu;
mod display;
mod keypad;

use cpu::*;
use display::Display;
use keypad::Keypad;

fn main() {
    let mut CPU = Cpu::new();

    loop {
        CPU.emulate_cycle(); // emulate one cycle
    }
}
