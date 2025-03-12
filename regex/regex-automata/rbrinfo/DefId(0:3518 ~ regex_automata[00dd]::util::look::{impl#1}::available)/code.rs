pub fn available(self) -> Result<(), UnicodeWordBoundaryError> {
        if self.contains_word_unicode() {
            UnicodeWordBoundaryError::check()?;
        }
        Ok(())
    }