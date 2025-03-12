fn any<F>(&mut self, f: F) -> bool
    where
        F: FnMut(Self::Item) -> bool,
    {
        wrap_either!(&mut self.inner => .any(f))
    }