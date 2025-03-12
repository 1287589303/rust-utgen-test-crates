pub fn build(&self, pattern: &str) -> Result<NFA, BuildError> {
        self.build_many(&[pattern])
    }