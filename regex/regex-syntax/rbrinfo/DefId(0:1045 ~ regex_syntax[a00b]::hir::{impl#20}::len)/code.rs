pub fn len(&self) -> usize {
        usize::from(self.end.checked_sub(self.start).unwrap())
            .checked_add(1)
            .unwrap()
    }