pub fn get_or_init<F>(&self, f: F) -> bool
    where
        F: FnOnce() -> bool,
    {
        OnceBool::from_usize(self.inner.get_or_init(|| OnceBool::to_usize(f())))
    }