pub fn octal(&mut self, yes: bool) -> &mut RegexSetBuilder {
            self.builder.octal(yes);
            self
        }