fn deref(&self) -> &T {
        self.value.as_deref().unwrap()
    }