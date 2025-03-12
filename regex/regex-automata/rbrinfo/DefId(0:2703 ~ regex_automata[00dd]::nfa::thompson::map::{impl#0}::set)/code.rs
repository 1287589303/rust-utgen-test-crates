pub fn set(
        &mut self,
        key: Vec<Transition>,
        hash: usize,
        state_id: StateID,
    ) {
        self.map[hash] =
            Utf8BoundedEntry { version: self.version, key, val: state_id };
    }