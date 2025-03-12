pub fn new_many<P: AsRef<str>>(
        patterns: &[P],
    ) -> Result<BoundedBacktracker, BuildError> {
        BoundedBacktracker::builder().build_many(patterns)
    }