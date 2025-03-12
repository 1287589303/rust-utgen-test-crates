fn next_capture_index(&self) -> Result<u32, Error> {
        let current = self.capture_index.get();
        let next = current
            .checked_add(1)
            .ok_or_else(|| Error::new(ERR_TOO_MANY_CAPTURES))?;
        self.capture_index.set(next);
        Ok(next)
    }