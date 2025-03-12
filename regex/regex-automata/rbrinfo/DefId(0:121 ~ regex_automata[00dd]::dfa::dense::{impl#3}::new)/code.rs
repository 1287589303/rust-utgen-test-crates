pub fn new(pattern: &str) -> Result<OwnedDFA, BuildError> {
        Builder::new().build(pattern)
    }