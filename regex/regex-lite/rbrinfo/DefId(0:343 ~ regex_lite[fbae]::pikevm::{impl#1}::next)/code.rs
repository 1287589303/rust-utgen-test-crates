fn next(&mut self) -> Option<(usize, usize)> {
        if !self.pikevm.search(
            &mut self.cache,
            self.haystack,
            self.at,
            self.haystack.len(),
            false,
            &mut self.slots,
        ) {
            return None;
        }
        let mut m =
            (self.slots[0].unwrap().get(), self.slots[1].unwrap().get());
        if m.0 >= m.1 {
            m = self.handle_overlapping_empty_match(m)?;
        }
        self.at = m.1;
        self.last_match_end = Some(m.1);
        Some(m)
    }