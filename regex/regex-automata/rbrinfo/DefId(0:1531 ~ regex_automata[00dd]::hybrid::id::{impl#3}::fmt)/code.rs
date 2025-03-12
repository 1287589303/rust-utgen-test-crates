fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(
            f,
            "failed to create LazyStateID from {:?}, which exceeds {:?}",
            self.attempted(),
            LazyStateID::MAX,
        )
    }