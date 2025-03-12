pub fn new(pattern: &str) -> Result<DFA<Vec<u8>>, BuildError> {
        dense::Builder::new()
            .build(pattern)
            .and_then(|dense| dense.to_sparse())
    }