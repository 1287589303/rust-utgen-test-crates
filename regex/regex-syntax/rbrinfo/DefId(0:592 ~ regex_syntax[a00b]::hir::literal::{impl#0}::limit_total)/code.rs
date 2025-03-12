pub fn limit_total(&mut self, limit: usize) -> &mut Extractor {
        self.limit_total = limit;
        self
    }