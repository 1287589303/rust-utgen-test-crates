pub fn build(
        &self,
        pattern: &str,
    ) -> Result<BoundedBacktracker, BuildError> {
        self.build_many(&[pattern])
    }