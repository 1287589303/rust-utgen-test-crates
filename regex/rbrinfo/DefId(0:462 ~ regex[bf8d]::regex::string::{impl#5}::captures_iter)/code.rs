pub fn captures_iter<'r, 'h>(
        &'r self,
        haystack: &'h str,
    ) -> CaptureMatches<'r, 'h> {
        CaptureMatches { haystack, it: self.meta.captures_iter(haystack) }
    }