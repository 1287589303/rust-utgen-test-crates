pub fn splitn<'r, 'h>(
        &'r self,
        haystack: &'h [u8],
        limit: usize,
    ) -> SplitN<'r, 'h> {
        SplitN { haystack, it: self.meta.splitn(haystack, limit) }
    }