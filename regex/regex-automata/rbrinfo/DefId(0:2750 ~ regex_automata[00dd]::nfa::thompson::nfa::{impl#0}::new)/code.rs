pub fn new(pattern: &str) -> Result<NFA, BuildError> {
        NFA::compiler().build(pattern)
    }