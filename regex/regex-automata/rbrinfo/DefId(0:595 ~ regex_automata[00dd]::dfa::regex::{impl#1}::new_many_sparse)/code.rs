pub fn new_many_sparse<P: AsRef<str>>(
        patterns: &[P],
    ) -> Result<Regex<sparse::DFA<Vec<u8>>>, BuildError> {
        Builder::new().build_many_sparse(patterns)
    }