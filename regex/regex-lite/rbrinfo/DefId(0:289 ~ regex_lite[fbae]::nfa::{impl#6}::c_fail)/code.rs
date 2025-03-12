fn c_fail(&self) -> Result<ThompsonRef, Error> {
        let id = self.add(State::Fail)?;
        Ok(ThompsonRef { start: id, end: id })
    }