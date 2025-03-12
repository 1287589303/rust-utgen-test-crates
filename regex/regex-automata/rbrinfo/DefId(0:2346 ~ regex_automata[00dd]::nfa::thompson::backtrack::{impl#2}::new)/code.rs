pub fn new(pattern: &str) -> Result<BoundedBacktracker, BuildError> {
        BoundedBacktracker::builder().build(pattern)
    }