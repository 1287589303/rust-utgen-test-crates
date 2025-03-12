pub fn set_union(&mut self, other: LookSet) {
        *self = self.union(other);
    }