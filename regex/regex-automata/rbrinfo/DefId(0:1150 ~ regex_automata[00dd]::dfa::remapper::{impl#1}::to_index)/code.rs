fn to_index(&self, id: StateID) -> usize {
        id.as_usize() >> self.stride2
    }