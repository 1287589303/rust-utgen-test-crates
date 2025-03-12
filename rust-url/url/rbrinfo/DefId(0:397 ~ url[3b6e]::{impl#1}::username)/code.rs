pub fn username(&self) -> &str {
        let scheme_separator_len = "://".len() as u32;
        if self.has_authority() && self.username_end > self.scheme_end + scheme_separator_len {
            self.slice(self.scheme_end + scheme_separator_len..self.username_end)
        } else {
            ""
        }
    }