pub const fn as_u64(&self) -> u64 {
        // FIXME: Use u64::from() once const functions in traits are stable.
        self.0 as u64
    }