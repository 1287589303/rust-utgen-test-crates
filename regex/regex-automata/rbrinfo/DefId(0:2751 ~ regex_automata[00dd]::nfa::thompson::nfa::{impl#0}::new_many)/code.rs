pub fn new_many<P: AsRef<str>>(patterns: &[P]) -> Result<NFA, BuildError> {
        NFA::compiler().build_many(patterns)
    }