fn group_len(&self, pid: PatternID) -> usize {
        let (start, end) = match self.slot_ranges.get(pid.as_usize()) {
            None => return 0,
            Some(range) => range,
        };
        // The difference between any two SmallIndex values always fits in a
        // usize since we know that SmallIndex::MAX <= isize::MAX-1. We also
        // know that start<=end by construction and that the number of groups
        // never exceeds SmallIndex and thus never overflows usize.
        1 + ((end.as_usize() - start.as_usize()) / 2)
    }