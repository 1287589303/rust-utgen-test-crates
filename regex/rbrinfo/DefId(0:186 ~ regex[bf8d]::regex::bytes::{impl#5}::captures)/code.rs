pub fn captures<'h>(&self, haystack: &'h [u8]) -> Option<Captures<'h>> {
        self.captures_at(haystack, 0)
    }