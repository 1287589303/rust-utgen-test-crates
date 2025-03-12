fn c_alternation<I>(&self, mut it: I) -> Result<ThompsonRef, Error>
    where
        I: Iterator<Item = Result<ThompsonRef, Error>>,
    {
        let first = match it.next() {
            None => return self.c_fail(),
            Some(result) => result?,
        };
        let second = match it.next() {
            None => return Ok(first),
            Some(result) => result?,
        };

        let splits =
            self.add(State::Splits { targets: vec![], reverse: false })?;
        let end = self.add_empty()?;
        self.patch(splits, first.start)?;
        self.patch(first.end, end)?;
        self.patch(splits, second.start)?;
        self.patch(second.end, end)?;
        for result in it {
            let compiled = result?;
            self.patch(splits, compiled.start)?;
            self.patch(compiled.end, end)?;
        }
        Ok(ThompsonRef { start: splits, end })
    }