pub fn replace<'h, R: Replacer>(
        &self,
        haystack: &'h [u8],
        rep: R,
    ) -> Cow<'h, [u8]> {
        self.replacen(haystack, 1, rep)
    }