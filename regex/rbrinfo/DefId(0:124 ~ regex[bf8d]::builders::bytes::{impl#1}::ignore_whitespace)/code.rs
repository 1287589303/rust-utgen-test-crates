pub fn ignore_whitespace(
            &mut self,
            yes: bool,
        ) -> &mut RegexSetBuilder {
            self.builder.ignore_whitespace(yes);
            self
        }