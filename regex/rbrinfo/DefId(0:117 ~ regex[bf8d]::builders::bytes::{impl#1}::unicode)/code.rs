pub fn unicode(&mut self, yes: bool) -> &mut RegexSetBuilder {
            self.builder.unicode(yes);
            self
        }