fn handle_overlapping_empty_match(
        &mut self,
        mut m: (usize, usize),
    ) -> Option<(usize, usize)> {
        assert!(m.0 >= m.1);
        if Some(m.1) == self.last_match_end {
            let len =
                core::cmp::max(1, utf8::decode(&self.haystack[self.at..]).1);
            self.at = self.at.checked_add(len).unwrap();
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
            m = (self.slots[0].unwrap().get(), self.slots[1].unwrap().get());
        }
        Some(m)
    }