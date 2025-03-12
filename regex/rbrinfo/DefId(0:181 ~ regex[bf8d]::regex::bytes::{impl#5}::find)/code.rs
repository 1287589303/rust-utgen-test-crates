pub fn find<'h>(&self, haystack: &'h [u8]) -> Option<Match<'h>> {
        self.find_at(haystack, 0)
    }