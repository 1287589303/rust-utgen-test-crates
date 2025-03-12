pub fn case_insensitive(&mut self, yes: bool) -> &mut RegexSetBuilder {
            self.builder.case_insensitive(yes);
            self
        }