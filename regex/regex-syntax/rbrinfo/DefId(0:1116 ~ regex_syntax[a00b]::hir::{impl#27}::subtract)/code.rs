pub fn subtract(self, other: LookSet) -> LookSet {
        LookSet { bits: self.bits & !other.bits }
    }