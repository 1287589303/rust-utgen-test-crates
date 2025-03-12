pub fn union(self, other: LookSet) -> LookSet {
        LookSet { bits: self.bits | other.bits }
    }