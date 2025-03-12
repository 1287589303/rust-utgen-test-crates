fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(
            f,
            "failed to create small index from {:?}, which exceeds {:?}",
            self.attempted(),
            SmallIndex::MAX,
        )
    }