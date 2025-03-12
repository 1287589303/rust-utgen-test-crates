pub fn minimum_len(&self) -> Option<usize> {
        if self.ranges().is_empty() {
            None
        } else {
            Some(1)
        }
    }