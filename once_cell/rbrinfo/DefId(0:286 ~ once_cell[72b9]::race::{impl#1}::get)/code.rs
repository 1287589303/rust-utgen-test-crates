pub fn get(&self) -> Option<bool> {
        self.inner.get().map(OnceBool::from_usize)
    }