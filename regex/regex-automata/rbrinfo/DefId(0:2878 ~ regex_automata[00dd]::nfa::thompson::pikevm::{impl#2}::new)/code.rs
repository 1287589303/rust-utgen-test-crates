pub fn new(pattern: &str) -> Result<PikeVM, BuildError> {
        PikeVM::builder().build(pattern)
    }