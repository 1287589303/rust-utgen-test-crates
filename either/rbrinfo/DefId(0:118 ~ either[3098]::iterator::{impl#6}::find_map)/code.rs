fn find_map<B, F>(&mut self, f: F) -> Option<B>
    where
        F: FnMut(Self::Item) -> Option<B>,
    {
        wrap_either!(&mut self.inner => .find_map(f))
    }