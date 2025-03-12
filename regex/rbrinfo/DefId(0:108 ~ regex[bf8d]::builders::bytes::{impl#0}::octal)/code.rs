pub fn octal(&mut self, yes: bool) -> &mut RegexBuilder {
            self.builder.octal(yes);
            self
        }