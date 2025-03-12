fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        crate::debug::Bytes(&self.0).fmt(f)
    }