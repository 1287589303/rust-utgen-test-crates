fn next(&mut self) -> Option<&'h [u8]> {
        self.it.next().map(|span| &self.haystack[span])
    }