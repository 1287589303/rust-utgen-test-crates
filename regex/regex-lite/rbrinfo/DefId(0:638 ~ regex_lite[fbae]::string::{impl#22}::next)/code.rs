fn next(&mut self) -> Option<&'h str> {
        match self.finder.next() {
            None => {
                let len = self.haystack.len();
                if self.last > len {
                    None
                } else {
                    let range = self.last..len;
                    self.last = len + 1; // Next call will return None
                    Some(&self.haystack[range])
                }
            }
            Some(m) => {
                let range = self.last..m.start();
                self.last = m.end();
                Some(&self.haystack[range])
            }
        }
    }