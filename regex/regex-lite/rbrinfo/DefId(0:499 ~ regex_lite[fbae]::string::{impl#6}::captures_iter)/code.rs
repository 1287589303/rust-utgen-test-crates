pub fn captures_iter<'r, 'h>(
        &'r self,
        haystack: &'h str,
    ) -> CaptureMatches<'r, 'h> {
        CaptureMatches {
            haystack,
            re: self,
            it: self
                .pikevm
                .captures_iter(self.pool.get(), haystack.as_bytes()),
        }
    }