fn c_empty(&self) -> Result<ThompsonRef, BuildError> {
        let id = self.add_empty()?;
        Ok(ThompsonRef { start: id, end: id })
    }