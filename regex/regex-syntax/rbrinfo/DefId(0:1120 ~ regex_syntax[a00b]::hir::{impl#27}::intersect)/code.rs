pub fn intersect(self, other: LookSet) -> LookSet {
        LookSet { bits: self.bits & other.bits }
    }