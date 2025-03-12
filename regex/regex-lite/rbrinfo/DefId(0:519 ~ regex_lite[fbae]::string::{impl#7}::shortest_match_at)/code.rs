pub fn shortest_match_at(
        &self,
        haystack: &str,
        start: usize,
    ) -> Option<usize> {
        let mut cache = self.pool.get();
        let mut slots = [None, None];
        let matched = self.pikevm.search(
            &mut cache,
            haystack.as_bytes(),
            start,
            haystack.len(),
            true,
            &mut slots,
        );
        if !matched {
            return None;
        }
        Some(slots[1].unwrap().get())
    }