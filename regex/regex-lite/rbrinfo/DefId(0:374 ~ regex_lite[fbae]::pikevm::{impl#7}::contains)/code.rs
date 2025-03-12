fn contains(&self, id: StateID) -> bool {
        let index = self.sparse[id.as_usize()];
        index.as_usize() < self.len() && self.dense[index.as_usize()] == id
    }