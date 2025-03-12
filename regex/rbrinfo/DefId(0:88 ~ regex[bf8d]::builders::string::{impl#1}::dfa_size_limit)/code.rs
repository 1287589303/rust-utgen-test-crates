pub fn dfa_size_limit(
            &mut self,
            bytes: usize,
        ) -> &mut RegexSetBuilder {
            self.builder.dfa_size_limit(bytes);
            self
        }