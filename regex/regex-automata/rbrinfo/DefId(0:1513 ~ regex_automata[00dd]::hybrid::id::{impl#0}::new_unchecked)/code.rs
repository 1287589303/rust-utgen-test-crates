const fn new_unchecked(id: usize) -> LazyStateID {
        // FIXME: Use as_u32() once const functions in traits are stable.
        LazyStateID(id as u32)
    }