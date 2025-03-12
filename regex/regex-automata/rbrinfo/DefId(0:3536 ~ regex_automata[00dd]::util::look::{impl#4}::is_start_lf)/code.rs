pub fn is_start_lf(&self, haystack: &[u8], at: usize) -> bool {
        self.is_start(haystack, at) || haystack[at - 1] == self.lineterm.0
    }