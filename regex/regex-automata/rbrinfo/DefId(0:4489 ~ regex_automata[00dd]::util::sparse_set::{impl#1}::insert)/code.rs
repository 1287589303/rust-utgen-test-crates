pub(crate) fn insert(&mut self, id: StateID) -> bool {
        if self.contains(id) {
            return false;
        }

        let i = self.len();
        assert!(
            i < self.capacity(),
            "{:?} exceeds capacity of {:?} when inserting {:?}",
            i,
            self.capacity(),
            id,
        );
        // OK since i < self.capacity() and self.capacity() is guaranteed to
        // be <= StateID::LIMIT.
        let index = StateID::new_unchecked(i);
        self.dense[index] = id;
        self.sparse[id] = index;
        self.len += 1;
        true
    }