pub struct Display {
    pub d:  bool,
}

impl Display {
    pub fn cls(&mut self) {
        self.d = false;
    }
}
