pub fn crlf(&mut self, yes: bool) -> &mut RegexSetBuilder {
            self.builder.crlf(yes);
            self
        }