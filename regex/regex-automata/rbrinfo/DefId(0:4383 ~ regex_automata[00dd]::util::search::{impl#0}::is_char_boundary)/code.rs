pub fn is_char_boundary(&self, offset: usize) -> bool {
        utf8::is_boundary(self.haystack(), offset)
    }