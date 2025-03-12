pub fn find_iter<'r, 'h>(&'r self, haystack: &'h [u8]) -> Matches<'r, 'h> {
        Matches { haystack, it: self.meta.find_iter(haystack) }
    }