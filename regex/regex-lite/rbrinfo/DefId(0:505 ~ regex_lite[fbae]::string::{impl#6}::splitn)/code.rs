pub fn splitn<'r, 'h>(
        &'r self,
        haystack: &'h str,
        limit: usize,
    ) -> SplitN<'r, 'h> {
        SplitN { splits: self.split(haystack), limit }
    }