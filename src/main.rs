mod cpu;
mod display;
mod keypad;

use cpu::*;
use display::Display;
use keypad::Keypad;

use std::fs;

fn main() {
    let mut cpu = Cpu::new();
    let filename = "c8_test.c8";

    let rom = fs::read(&filename).expect("Unable to read file");
    cpu.load_rom(&rom);

    loop {
        cpu.emulate_cycle();
    }
}
