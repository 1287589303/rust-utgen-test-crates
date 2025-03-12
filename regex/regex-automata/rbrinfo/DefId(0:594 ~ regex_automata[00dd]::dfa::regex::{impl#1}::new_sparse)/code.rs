pub fn new_sparse(
        pattern: &str,
    ) -> Result<Regex<sparse::DFA<Vec<u8>>>, BuildError> {
        Builder::new().build_sparse(pattern)
    }