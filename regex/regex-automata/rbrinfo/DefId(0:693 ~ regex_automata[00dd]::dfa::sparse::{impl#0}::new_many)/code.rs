pub fn new_many<P: AsRef<str>>(
        patterns: &[P],
    ) -> Result<DFA<Vec<u8>>, BuildError> {
        dense::Builder::new()
            .build_many(patterns)
            .and_then(|dense| dense.to_sparse())
    }