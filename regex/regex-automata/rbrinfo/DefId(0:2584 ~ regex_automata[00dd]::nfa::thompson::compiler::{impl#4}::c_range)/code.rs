fn c_range(&self, start: u8, end: u8) -> Result<ThompsonRef, BuildError> {
        let id = self.add_range(start, end)?;
        Ok(ThompsonRef { start: id, end: id })
    }