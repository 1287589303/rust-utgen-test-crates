pub fn size_limit(&mut self, bytes: usize) -> &mut RegexSetBuilder {
            self.builder.size_limit(bytes);
            self
        }