pub fn set_subtract(&mut self, other: LookSet) {
        *self = self.subtract(other);
    }