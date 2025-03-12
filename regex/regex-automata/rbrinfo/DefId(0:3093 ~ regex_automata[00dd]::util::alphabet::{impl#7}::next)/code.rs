fn next(&mut self) -> Option<Unit> {
        while self.byte < 256 {
            let byte = u8::try_from(self.byte).unwrap();
            self.byte += 1;
            if self.class.is_byte(self.classes.get(byte)) {
                return Some(Unit::u8(byte));
            }
        }
        if self.byte < 257 {
            self.byte += 1;
            if self.class.is_eoi() {
                return Some(Unit::eoi(256));
            }
        }
        None
    }