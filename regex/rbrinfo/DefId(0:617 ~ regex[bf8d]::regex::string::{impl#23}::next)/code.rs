fn next(&mut self) -> Option<&'h str> {
        self.it.next().map(|span| &self.haystack[span])
    }