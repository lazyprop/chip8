pub struct Keypad {
    pub keys: [bool; 16],
}

impl Keypad {
    pub fn new() -> Keypad {
        Keypad { keys: [false; 16] }
    }

    pub fn is_pressed(&self, index: u8) -> bool {
        self.keys[index as usize]
    }
}
