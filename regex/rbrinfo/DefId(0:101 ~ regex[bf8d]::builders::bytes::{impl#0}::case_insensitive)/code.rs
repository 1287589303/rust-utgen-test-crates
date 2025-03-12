pub fn case_insensitive(&mut self, yes: bool) -> &mut RegexBuilder {
            self.builder.case_insensitive(yes);
            self
        }