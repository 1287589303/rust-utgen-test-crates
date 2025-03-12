pub fn crlf(&mut self, yes: bool) -> &mut RegexBuilder {
            self.builder.crlf(yes);
            self
        }