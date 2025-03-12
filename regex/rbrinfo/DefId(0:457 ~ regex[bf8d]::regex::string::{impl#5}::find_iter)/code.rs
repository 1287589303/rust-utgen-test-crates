pub fn find_iter<'r, 'h>(&'r self, haystack: &'h str) -> Matches<'r, 'h> {
        Matches { haystack, it: self.meta.find_iter(haystack) }
    }