pub fn set(&self, value: bool) -> Result<(), ()> {
        self.inner.set(OnceBool::to_usize(value))
    }