pub fn pop(&mut self) -> &mut Self {
        if self.after_first_slash >= self.url.serialization.len() {
            return self;
        }
        let last_slash = self.url.serialization[self.after_first_slash..]
            .rfind('/')
            .unwrap_or(0);
        self.url
            .serialization
            .truncate(self.after_first_slash + last_slash);
        self
    }