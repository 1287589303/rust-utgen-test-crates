pub fn multi_line(&mut self, yes: bool) -> &mut RegexSetBuilder {
            self.builder.multi_line(yes);
            self
        }