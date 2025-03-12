pub fn as_str(&self) -> &'h str {
        &self.haystack[self.range()]
    }