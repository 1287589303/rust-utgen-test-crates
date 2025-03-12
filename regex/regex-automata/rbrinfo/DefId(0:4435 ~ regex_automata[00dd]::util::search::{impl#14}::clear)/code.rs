pub fn clear(&mut self) {
        self.len = 0;
        for matched in self.which.iter_mut() {
            *matched = false;
        }
    }