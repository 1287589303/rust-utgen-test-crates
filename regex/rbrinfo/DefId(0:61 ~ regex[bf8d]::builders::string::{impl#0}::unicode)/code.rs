pub fn unicode(&mut self, yes: bool) -> &mut RegexBuilder {
            self.builder.unicode(yes);
            self
        }