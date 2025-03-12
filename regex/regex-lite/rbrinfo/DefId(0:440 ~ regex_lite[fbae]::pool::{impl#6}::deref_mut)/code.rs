fn deref_mut(&mut self) -> &mut T {
        self.value.as_deref_mut().unwrap()
    }