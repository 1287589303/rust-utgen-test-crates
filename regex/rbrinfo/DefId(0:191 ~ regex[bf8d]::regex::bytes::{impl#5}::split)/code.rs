pub fn split<'r, 'h>(&'r self, haystack: &'h [u8]) -> Split<'r, 'h> {
        Split { haystack, it: self.meta.split(haystack) }
    }