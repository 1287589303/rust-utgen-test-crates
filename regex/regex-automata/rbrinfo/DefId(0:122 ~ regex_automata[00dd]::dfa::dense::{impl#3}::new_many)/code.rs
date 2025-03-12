pub fn new_many<P: AsRef<str>>(
        patterns: &[P],
    ) -> Result<OwnedDFA, BuildError> {
        Builder::new().build_many(patterns)
    }