pub const fn as_i32(&self) -> i32 {
        // This is OK because we guarantee that our max value is <= i32::MAX.
        self.0 as i32
    }