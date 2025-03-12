fn next(&mut self) -> Option<Unit> {
        if self.i + 1 == self.classes.alphabet_len() {
            self.i += 1;
            Some(self.classes.eoi())
        } else if self.i < self.classes.alphabet_len() {
            let class = u8::try_from(self.i).unwrap();
            self.i += 1;
            Some(Unit::u8(class))
        } else {
            None
        }
    }