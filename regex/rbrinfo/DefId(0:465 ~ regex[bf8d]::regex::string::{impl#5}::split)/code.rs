pub fn split<'r, 'h>(&'r self, haystack: &'h str) -> Split<'r, 'h> {
        Split { haystack, it: self.meta.split(haystack) }
    }