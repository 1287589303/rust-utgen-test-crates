fn next(&mut self) -> Option<Option<Span>> {
        let (group_index, _) = self.names.next()?;
        Some(self.caps.get_group(group_index))
    }