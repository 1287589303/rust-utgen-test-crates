pub fn swap_greed(&mut self, yes: bool) -> &mut RegexSetBuilder {
            self.builder.swap_greed(yes);
            self
        }