pub fn swap_greed(&mut self, yes: bool) -> &mut RegexBuilder {
            self.builder.swap_greed(yes);
            self
        }