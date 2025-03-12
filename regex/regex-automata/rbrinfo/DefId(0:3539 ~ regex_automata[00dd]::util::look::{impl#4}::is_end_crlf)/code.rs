pub fn is_end_crlf(&self, haystack: &[u8], at: usize) -> bool {
        self.is_end(haystack, at)
            || haystack[at] == b'\r'
            || (haystack[at] == b'\n'
                && (at == 0 || haystack[at - 1] != b'\r'))
    }