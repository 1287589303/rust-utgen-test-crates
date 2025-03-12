pub fn len(&self) -> usize {
        self.pikevm.nfa().group_len()
    }