fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("ClassBytesRange")
            .field("start", &crate::debug::Byte(self.start))
            .field("end", &crate::debug::Byte(self.end))
            .finish()
    }