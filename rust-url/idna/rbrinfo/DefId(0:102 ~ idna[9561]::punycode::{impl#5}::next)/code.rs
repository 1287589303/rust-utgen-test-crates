fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.insertions.get(self.inserted) {
                Some((pos, c)) if *pos == self.position => {
                    self.inserted += 1;
                    self.position += 1;
                    return Some(*c);
                }
                _ => {}
            }
            if let Some(c) = self.base.next() {
                self.position += 1;
                return Some(if C::EXTERNAL_CALLER {
                    c.char()
                } else {
                    c.char_ascii_lower_case()
                });
            } else if self.inserted >= self.insertions.len() {
                return None;
            }
        }
    }