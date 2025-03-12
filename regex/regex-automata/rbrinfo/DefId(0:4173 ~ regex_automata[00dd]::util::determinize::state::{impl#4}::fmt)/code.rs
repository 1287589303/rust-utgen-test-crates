fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_tuple("StateBuilderMatches").field(&self.repr()).finish()
    }