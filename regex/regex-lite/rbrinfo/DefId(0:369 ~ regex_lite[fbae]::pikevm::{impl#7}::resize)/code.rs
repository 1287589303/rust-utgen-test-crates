fn resize(&mut self, new_capacity: usize) {
        assert!(
            new_capacity <= u32::MAX.as_usize(),
            "sparse set capacity cannot excced {:?}",
            u32::MAX,
        );
        self.clear();
        self.dense.resize(new_capacity, 0);
        self.sparse.resize(new_capacity, 0);
    }