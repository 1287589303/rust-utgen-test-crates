pub fn replace<'h, R: Replacer>(
        &self,
        haystack: &'h str,
        rep: R,
    ) -> Cow<'h, str> {
        self.replacen(haystack, 1, rep)
    }