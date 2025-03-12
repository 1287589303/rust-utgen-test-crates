pub fn size_limit(&mut self, bytes: usize) -> &mut RegexBuilder {
            self.builder.size_limit(bytes);
            self
        }