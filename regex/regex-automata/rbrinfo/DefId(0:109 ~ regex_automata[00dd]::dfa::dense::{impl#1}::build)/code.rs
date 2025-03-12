pub fn build(&self, pattern: &str) -> Result<OwnedDFA, BuildError> {
        self.build_many(&[pattern])
    }