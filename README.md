# Chip-8 Emulator in Rust

### TODO
- Deal with keyboard input.

### Resources
- [Opcode Table](https://en.wikipedia.org/wiki/CHIP-8#Opcode_table)
- [Cowgod's Guide](http://devernay.free.fr/hacks/chip8/C8TECH10.HTM#0.1)
- [Multigesture's Guide](http://www.multigesture.net/articles/how-to-write-an-emulator-chip-8-interpreter/)
- Unit tests for `Cpu` copied from [ScottLogic's implementation](https://github.com/ColinEberhardt/wasm-rust-chip8/blob/master/src/cpu.rs#L222)
- `c8_test.c8` test ROM copied from
  [Skosulor/c8int](https://github.com/Skosulor/c8int/tree/master/test).
  final instruction `0x13a8` or `JP 0x03a8` is at `0x03a8` hence goes into
  infinite loop.
- `Sierpinski.ch8` copied from [here](https://github.com/dmatlack/chip8/blob/master/roms/demos/Sierpinski%20%5BSergey%20Naydenov%2C%202010%5D.ch8).
