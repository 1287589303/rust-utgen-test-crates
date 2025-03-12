pub fn captures_len(&self) -> usize {
        self.pikevm.nfa().group_len()
    }