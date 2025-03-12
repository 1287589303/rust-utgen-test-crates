pub fn is_start_crlf(&self, haystack: &[u8], at: usize) -> bool {
        self.is_start(haystack, at)
            || haystack[at - 1] == b'\n'
            || (haystack[at - 1] == b'\r'
                && (at >= haystack.len() || haystack[at] != b'\n'))
    }