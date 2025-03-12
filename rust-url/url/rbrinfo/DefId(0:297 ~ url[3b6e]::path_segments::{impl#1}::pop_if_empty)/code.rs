pub fn pop_if_empty(&mut self) -> &mut Self {
        if self.after_first_slash >= self.url.serialization.len() {
            return self;
        }
        if self.url.serialization[self.after_first_slash..].ends_with('/') {
            self.url.serialization.pop();
        }
        self
    }