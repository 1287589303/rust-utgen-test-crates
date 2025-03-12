fn c_char(&self, ch: char) -> Result<ThompsonRef, Error> {
        let id = self.add(State::Char { target: 0, ch })?;
        Ok(ThompsonRef { start: id, end: id })
    }