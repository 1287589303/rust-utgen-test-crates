fn c_concat<I>(&self, mut it: I) -> Result<ThompsonRef, Error>
    where
        I: Iterator<Item = Result<ThompsonRef, Error>>,
    {
        let ThompsonRef { start, mut end } = match it.next() {
            Some(result) => result?,
            None => return self.c_empty(),
        };
        for result in it {
            let compiled = result?;
            self.patch(end, compiled.start)?;
            end = compiled.end;
        }
        Ok(ThompsonRef { start, end })
    }