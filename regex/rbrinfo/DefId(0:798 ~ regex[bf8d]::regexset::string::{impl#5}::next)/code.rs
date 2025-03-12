fn next(&mut self) -> Option<usize> {
        loop {
            let id = self.it.next()?;
            if self.patset.contains(PatternID::new_unchecked(id)) {
                return Some(id);
            }
        }
    }