fn decrement(self) -> Self {
        self.checked_sub(1).unwrap()
    }