pub(crate) fn as_u8(&self) -> u8 {
        // AFAIK, 'as' is the only way to zero-cost convert an int enum to an
        // actual int.
        *self as u8
    }