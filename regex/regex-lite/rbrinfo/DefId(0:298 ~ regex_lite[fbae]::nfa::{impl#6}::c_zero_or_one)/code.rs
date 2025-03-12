fn c_zero_or_one(
        &self,
        hir: &Hir,
        greedy: bool,
    ) -> Result<ThompsonRef, Error> {
        let splits =
            self.add(State::Splits { targets: vec![], reverse: !greedy })?;
        let compiled = self.c(hir)?;
        let empty = self.add_empty()?;
        self.patch(splits, compiled.start)?;
        self.patch(splits, empty)?;
        self.patch(compiled.end, empty)?;
        Ok(ThompsonRef { start: splits, end: empty })
    }