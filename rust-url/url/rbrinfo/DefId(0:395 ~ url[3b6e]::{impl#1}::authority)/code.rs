pub fn authority(&self) -> &str {
        let scheme_separator_len = "://".len() as u32;
        if self.has_authority() && self.path_start > self.scheme_end + scheme_separator_len {
            self.slice(self.scheme_end + scheme_separator_len..self.path_start)
        } else {
            ""
        }
    }