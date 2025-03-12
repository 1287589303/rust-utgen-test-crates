pub fn captures_iter<'r, 'h>(
        &'r self,
        haystack: &'h [u8],
    ) -> CaptureMatches<'r, 'h> {
        CaptureMatches { haystack, it: self.meta.captures_iter(haystack) }
    }