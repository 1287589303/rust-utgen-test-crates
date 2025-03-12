pub fn find_iter<'r, 'h>(&'r self, haystack: &'h str) -> Matches<'r, 'h> {
        Matches {
            haystack,
            it: self.pikevm.find_iter(self.pool.get(), haystack.as_bytes()),
        }
    }