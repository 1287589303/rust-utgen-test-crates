pub(crate) fn contains(&self, id: StateID) -> bool {
        let index = self.sparse[id];
        index.as_usize() < self.len() && self.dense[index] == id
    }