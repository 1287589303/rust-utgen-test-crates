fn count(self) -> usize {
        // This can actually be up to 2x faster than calling `next()` until
        // completion, because counting matches when using a DFA only requires
        // finding the end of each match. But returning a `Match` via `next()`
        // requires the start of each match which, with a DFA, requires a
        // reverse forward scan to find it.
        self.it.count()
    }