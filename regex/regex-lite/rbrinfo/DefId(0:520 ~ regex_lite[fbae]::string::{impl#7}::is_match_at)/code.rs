pub fn is_match_at(&self, haystack: &str, start: usize) -> bool {
        let mut cache = self.pool.get();
        self.pikevm.search(
            &mut cache,
            haystack.as_bytes(),
            start,
            haystack.len(),
            true,
            &mut [],
        )
    }