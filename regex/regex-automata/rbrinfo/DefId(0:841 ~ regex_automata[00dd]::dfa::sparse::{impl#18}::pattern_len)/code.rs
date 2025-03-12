fn pattern_len(&self) -> usize {
        assert_eq!(0, self.pattern_ids.len() % 4);
        self.pattern_ids.len() / 4
    }