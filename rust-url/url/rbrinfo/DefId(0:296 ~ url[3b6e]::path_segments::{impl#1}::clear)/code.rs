pub fn clear(&mut self) -> &mut Self {
        self.url.serialization.truncate(self.after_first_slash);
        self
    }