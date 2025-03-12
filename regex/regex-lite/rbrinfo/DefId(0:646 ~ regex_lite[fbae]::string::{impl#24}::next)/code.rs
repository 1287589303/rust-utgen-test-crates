fn next(&mut self) -> Option<&'h str> {
        if self.limit == 0 {
            return None;
        }

        self.limit -= 1;
        if self.limit > 0 {
            return self.splits.next();
        }

        let len = self.splits.haystack.len();
        if self.splits.last > len {
            // We've already returned all substrings.
            None
        } else {
            // self.n == 0, so future calls will return None immediately
            Some(&self.splits.haystack[self.splits.last..len])
        }
    }