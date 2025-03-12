pub fn is_empty(&self) -> bool {
        self.clone().next().is_none()
    }