fn increment(self) -> Self {
        self.checked_add(1).unwrap()
    }