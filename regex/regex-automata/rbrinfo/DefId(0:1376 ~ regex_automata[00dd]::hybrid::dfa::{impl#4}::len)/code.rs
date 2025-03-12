fn len(&self) -> usize {
        if self.start <= self.at {
            self.at - self.start
        } else {
            self.start - self.at
        }
    }