pub fn split<'r, 'h>(&'r self, haystack: &'h str) -> Split<'r, 'h> {
        Split { haystack, finder: self.find_iter(haystack), last: 0 }
    }