pub fn new_many<P: AsRef<str>>(
        patterns: &[P],
    ) -> Result<PikeVM, BuildError> {
        PikeVM::builder().build_many(patterns)
    }