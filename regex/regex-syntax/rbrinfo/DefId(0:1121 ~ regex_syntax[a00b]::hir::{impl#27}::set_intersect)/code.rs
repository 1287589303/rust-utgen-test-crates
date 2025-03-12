pub fn set_intersect(&mut self, other: LookSet) {
        *self = self.intersect(other);
    }