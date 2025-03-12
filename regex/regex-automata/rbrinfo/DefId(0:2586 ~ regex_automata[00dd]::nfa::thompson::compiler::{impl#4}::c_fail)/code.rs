fn c_fail(&self) -> Result<ThompsonRef, BuildError> {
        let id = self.add_fail()?;
        Ok(ThompsonRef { start: id, end: id })
    }