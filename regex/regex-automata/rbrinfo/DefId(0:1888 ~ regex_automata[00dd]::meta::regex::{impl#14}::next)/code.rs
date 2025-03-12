fn next(&mut self) -> Option<Span> {
        match self.finder.next() {
            None => {
                let len = self.finder.it.input().haystack().len();
                if self.last > len {
                    None
                } else {
                    let span = Span::from(self.last..len);
                    self.last = len + 1; // Next call will return None
                    Some(span)
                }
            }
            Some(m) => {
                let span = Span::from(self.last..m.start());
                self.last = m.end();
                Some(span)
            }
        }
    }