fn next(&mut self) -> Option<Option<&'a str>> {
        self.it.next().map(|n| n.as_deref())
    }