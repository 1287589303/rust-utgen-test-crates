pub fn multi_line(&mut self, yes: bool) -> &mut RegexBuilder {
            self.builder.multi_line(yes);
            self
        }