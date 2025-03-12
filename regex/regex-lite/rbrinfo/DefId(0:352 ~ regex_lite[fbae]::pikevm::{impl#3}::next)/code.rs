fn next(&mut self) -> Option<Vec<Option<NonMaxUsize>>> {
        self.it.next()?;
        Some(self.it.slots.clone())
    }