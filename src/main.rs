mod cpu;
mod display;
mod keypad;

use cpu::*;
use display::Display;
use keypad::Keypad;

fn main() {
    let mut cpu = Cpu::new();

    loop {
        cpu.emulate_cycle(); // emulate one cycle
    }
}
