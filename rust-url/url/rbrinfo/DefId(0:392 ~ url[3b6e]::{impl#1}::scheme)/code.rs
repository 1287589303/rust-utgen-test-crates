pub fn scheme(&self) -> &str {
        self.slice(..self.scheme_end)
    }