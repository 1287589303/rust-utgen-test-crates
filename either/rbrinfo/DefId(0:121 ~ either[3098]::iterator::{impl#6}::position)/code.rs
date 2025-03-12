fn position<P>(&mut self, predicate: P) -> Option<usize>
    where
        P: FnMut(Self::Item) -> bool,
    {
        wrap_either!(&mut self.inner => .position(predicate))
    }