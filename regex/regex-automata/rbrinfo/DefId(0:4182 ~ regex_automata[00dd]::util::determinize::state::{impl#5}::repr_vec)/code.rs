fn repr_vec(&mut self) -> ReprVec<'_> {
        ReprVec(&mut self.0)
    }