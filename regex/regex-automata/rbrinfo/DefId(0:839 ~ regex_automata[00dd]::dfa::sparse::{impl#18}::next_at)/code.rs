fn next_at(&self, i: usize) -> StateID {
        let start = i * StateID::SIZE;
        let end = start + StateID::SIZE;
        let bytes = self.next[start..end].try_into().unwrap();
        StateID::from_ne_bytes_unchecked(bytes)
    }