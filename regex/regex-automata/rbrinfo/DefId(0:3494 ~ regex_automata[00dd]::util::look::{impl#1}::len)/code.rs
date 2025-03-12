pub fn len(self) -> usize {
        // OK because max value always fits in a u8, which in turn always
        // fits in a usize, regardless of target.
        usize::try_from(self.bits.count_ones()).unwrap()
    }