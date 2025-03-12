fn next(&mut self) -> Option<u8> {
        while self.b <= 255 {
            let b = u8::try_from(self.b).unwrap();
            self.b += 1;
            if self.set.contains(b) {
                return Some(b);
            }
        }
        None
    }