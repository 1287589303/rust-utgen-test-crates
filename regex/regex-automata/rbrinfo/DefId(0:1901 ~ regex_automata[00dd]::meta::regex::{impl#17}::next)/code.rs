fn next(&mut self) -> Option<Span> {
        if self.limit == 0 {
            return None;
        }

        self.limit -= 1;
        if self.limit > 0 {
            return self.splits.next();
        }

        let len = self.splits.finder.it.input().haystack().len();
        if self.splits.last > len {
            // We've already returned all substrings.
            None
        } else {
            // self.n == 0, so future calls will return None immediately
            Some(Span::from(self.splits.last..len))
        }
    }