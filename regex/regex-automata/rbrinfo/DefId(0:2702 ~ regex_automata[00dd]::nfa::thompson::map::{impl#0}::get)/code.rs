pub fn get(&mut self, key: &[Transition], hash: usize) -> Option<StateID> {
        let entry = &self.map[hash];
        if entry.version != self.version {
            return None;
        }
        // There may be a hash collision, so we need to confirm real equality.
        if entry.key != key {
            return None;
        }
        Some(entry.val)
    }