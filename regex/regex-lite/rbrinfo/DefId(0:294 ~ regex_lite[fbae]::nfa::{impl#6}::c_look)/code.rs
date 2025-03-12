fn c_look(&self, look: &hir::Look) -> Result<ThompsonRef, Error> {
        let id = self.add(State::Goto { target: 0, look: Some(*look) })?;
        Ok(ThompsonRef { start: id, end: id })
    }