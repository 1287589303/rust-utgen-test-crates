pub fn new(pattern: &str) -> Result<Regex, BuildError> {
        Builder::new().build(pattern)
    }