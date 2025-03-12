fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        let tag = if self.exact { "E" } else { "I" };
        f.debug_tuple(tag)
            .field(&crate::debug::Bytes(self.as_bytes()))
            .finish()
    }