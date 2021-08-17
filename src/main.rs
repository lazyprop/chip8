mod cpu;
mod display;
mod keypad;

use cpu::*;
use display::{Display, HEIGHT, WIDTH};
use keypad::Keypad;

use minifb::{Key, Scale, Window, WindowOptions};
use std::fs;

fn main() {
    let mut cpu = Cpu::new();
    let filename = "c8_test.c8";
    //let filename = "sierpinski.ch8";

    let rom = fs::read(&filename).expect("Unable to read file");
    cpu.load_rom(&rom);

    // setup windows
    let mut window = Window::new(
        "CHIP-8",
        WIDTH,
        HEIGHT,
        WindowOptions {
            scale: Scale::X16,
            ..WindowOptions::default()
        },
    )
    .expect("Unable to open window");

    let display_refresh_rate: f64 = 500.0;
    let runloop_timer_default: usize = (display_refresh_rate / 60.0) as usize;
    window.limit_update_rate(Some(std::time::Duration::from_secs_f64(
        1.0 / display_refresh_rate,
    )));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        cpu.emulate_cycle();

        window.update_with_buffer(&cpu.display.memory, WIDTH, HEIGHT);
    }
}
