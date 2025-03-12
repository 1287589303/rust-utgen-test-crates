pub fn build(&self, pattern: &str) -> Result<PikeVM, BuildError> {
        self.build_many(&[pattern])
    }