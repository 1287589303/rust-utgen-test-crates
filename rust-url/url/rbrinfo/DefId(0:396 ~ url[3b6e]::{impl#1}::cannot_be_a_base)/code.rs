pub fn cannot_be_a_base(&self) -> bool {
        !self.slice(self.scheme_end + 1..).starts_with('/')
    }