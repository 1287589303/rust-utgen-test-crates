fn next(&mut self) -> Option<Option<Match<'h>>> {
        let (group_index, _) = self.it.next()?;
        Some(self.caps.get(group_index))
    }