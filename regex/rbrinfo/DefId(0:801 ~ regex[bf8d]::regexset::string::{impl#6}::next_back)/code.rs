fn next_back(&mut self) -> Option<usize> {
        loop {
            let id = self.it.next_back()?;
            if self.patset.contains(PatternID::new_unchecked(id)) {
                return Some(id);
            }
        }
    }