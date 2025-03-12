pub const fn as_repr(self) -> u32 {
        // AFAIK, 'as' is the only way to zero-cost convert an int enum to an
        // actual int.
        self as u32
    }