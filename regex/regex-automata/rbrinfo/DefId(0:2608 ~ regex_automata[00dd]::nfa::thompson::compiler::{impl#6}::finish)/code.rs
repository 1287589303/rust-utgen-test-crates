fn finish(&mut self) -> Result<ThompsonRef, BuildError> {
        self.compile_from(0)?;
        let node = self.pop_root();
        let start = self.compile(node)?;
        Ok(ThompsonRef { start, end: self.target })
    }