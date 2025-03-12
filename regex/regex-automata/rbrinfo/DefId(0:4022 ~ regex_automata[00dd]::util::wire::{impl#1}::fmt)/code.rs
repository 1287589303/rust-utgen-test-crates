fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "destination buffer is too small to write {}", self.what)
    }