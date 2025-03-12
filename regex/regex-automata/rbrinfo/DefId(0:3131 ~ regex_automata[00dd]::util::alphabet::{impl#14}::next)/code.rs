fn next(&mut self) -> Option<(u8, u8)> {
        let asu8 = |n: usize| u8::try_from(n).unwrap();
        while self.b <= 255 {
            let start = asu8(self.b);
            self.b += 1;
            if !self.set.contains(start) {
                continue;
            }

            let mut end = start;
            while self.b <= 255 && self.set.contains(asu8(self.b)) {
                end = asu8(self.b);
                self.b += 1;
            }
            return Some((start, end));
        }
        None
    }