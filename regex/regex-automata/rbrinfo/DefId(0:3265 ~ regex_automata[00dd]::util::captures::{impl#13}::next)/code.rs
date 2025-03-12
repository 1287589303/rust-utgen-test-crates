fn next(&mut self) -> Option<Option<&'a str>> {
        self.it.next().map(|x| x.as_deref())
    }