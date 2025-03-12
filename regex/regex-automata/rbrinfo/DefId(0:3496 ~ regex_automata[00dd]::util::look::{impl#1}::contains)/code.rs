pub fn contains(self, look: Look) -> bool {
        self.bits & look.as_repr() != 0
    }