pub fn ignore_whitespace(&mut self, yes: bool) -> &mut RegexBuilder {
            self.builder.ignore_whitespace(yes);
            self
        }