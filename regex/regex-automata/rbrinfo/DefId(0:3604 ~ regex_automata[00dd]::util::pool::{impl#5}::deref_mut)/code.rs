fn deref_mut(&mut self) -> &mut T {
        self.0.value_mut()
    }