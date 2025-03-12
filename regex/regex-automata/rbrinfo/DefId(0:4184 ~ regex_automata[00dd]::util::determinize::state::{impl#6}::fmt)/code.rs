fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_tuple("StateBuilderNFA").field(&self.repr()).finish()
    }