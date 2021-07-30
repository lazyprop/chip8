const WIDTH: usize = 64;
const HEIGHT: usize = 32;

pub struct Display {
    pub memory: [[u8; WIDTH]; HEIGHT],
}

impl Display {
    pub fn new() -> Display {
        Display { memory: [[0; WIDTH]; HEIGHT] }
    }

    pub fn cls(&mut self) {
        for x in 0..WIDTH {
            for y in 0..HEIGHT {
		self.memory[x][y] = 0;
            }
        }
    }

    pub fn draw(&mut self, x: usize, y: usize, sprite: &[u8]) -> bool {
        let rows = sprite.len();
        let mut collision = false;

        for j in 0..rows {
            let row = sprite[j];
            for i in 0..8 {
                let new_value = row >> (7 - i) & 0x01;
                if new_value == 1 {
                    let xi = (x + i) % WIDTH;
                    let yj = (y + j) % HEIGHT;
                    let old_value = self.memory[xi][yj] == 1;
                    if old_value {
                        collision = true;
                    }
                    self.memory[xi][yj] = ((new_value == 1) ^ old_value) as u8;
                }
            }
        }

        collision
    }
}

pub static FONT_SET: [[u8; 5]; 16] = [
  [0xF0, 0x90, 0x90, 0x90, 0xF0], // 0
  [0x20, 0x60, 0x20, 0x20, 0x70], // 1
  [0xF0, 0x10, 0xF0, 0x80, 0xF0], // 2
  [0xF0, 0x10, 0xF0, 0x10, 0xF0], // 3
  [0x90, 0x90, 0xF0, 0x10, 0x10], // 4
  [0xF0, 0x80, 0xF0, 0x10, 0xF0], // 5
  [0xF0, 0x80, 0xF0, 0x90, 0xF0], // 6
  [0xF0, 0x10, 0x20, 0x40, 0x40], // 7
  [0xF0, 0x90, 0xF0, 0x90, 0xF0], // 8
  [0xF0, 0x90, 0xF0, 0x10, 0xF0], // 9
  [0xF0, 0x90, 0xF0, 0x90, 0x90], // A
  [0xE0, 0x90, 0xE0, 0x90, 0xE0], // B
  [0xF0, 0x80, 0x80, 0x80, 0xF0], // C
  [0xE0, 0x90, 0x90, 0x90, 0xE0], // D
  [0xF0, 0x80, 0xF0, 0x80, 0xF0], // E
  [0xF0, 0x80, 0xF0, 0x80, 0x80]  // F
];
