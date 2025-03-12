fn rfind<P>(&mut self, predicate: P) -> Option<Self::Item>
    where
        P: FnMut(&Self::Item) -> bool,
    {
        wrap_either!(&mut self.inner => .rfind(predicate))
    }