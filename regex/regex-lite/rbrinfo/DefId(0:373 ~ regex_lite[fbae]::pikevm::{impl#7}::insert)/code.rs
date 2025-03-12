fn insert(&mut self, id: StateID) -> bool {
        if self.contains(id) {
            return false;
        }

        let index = self.len();
        assert!(
            index < self.capacity(),
            "{:?} exceeds capacity of {:?} when inserting {:?}",
            index,
            self.capacity(),
            id,
        );
        self.dense[index] = id;
        // OK because we don't permit the capacity to be set higher than
        // u32::MAX.
        self.sparse[id.as_usize()] = u32::try_from(index).unwrap();
        self.len += 1;
        true
    }