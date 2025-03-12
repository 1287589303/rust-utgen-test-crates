pub const fn new_unchecked(index: usize) -> SmallIndex {
        // FIXME: Use as_u32() once const functions in traits are stable.
        SmallIndex(index as u32)
    }