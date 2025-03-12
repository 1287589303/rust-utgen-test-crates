pub fn set(&mut self, key: Utf8SuffixKey, hash: usize, state_id: StateID) {
        self.map[hash] =
            Utf8SuffixEntry { version: self.version, key, val: state_id };
    }