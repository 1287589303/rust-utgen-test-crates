pub fn nest_limit(&mut self, limit: u32) -> &mut RegexSetBuilder {
            self.builder.nest_limit(limit);
            self
        }