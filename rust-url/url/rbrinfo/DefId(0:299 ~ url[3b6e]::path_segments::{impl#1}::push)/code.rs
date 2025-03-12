pub fn push(&mut self, segment: &str) -> &mut Self {
        self.extend(Some(segment))
    }