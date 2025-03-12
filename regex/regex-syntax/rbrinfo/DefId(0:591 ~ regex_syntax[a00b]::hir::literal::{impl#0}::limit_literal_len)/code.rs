pub fn limit_literal_len(&mut self, limit: usize) -> &mut Extractor {
        self.limit_literal_len = limit;
        self
    }