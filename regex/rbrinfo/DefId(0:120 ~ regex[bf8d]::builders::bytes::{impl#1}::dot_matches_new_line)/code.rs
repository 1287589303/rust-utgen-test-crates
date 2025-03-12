pub fn dot_matches_new_line(
            &mut self,
            yes: bool,
        ) -> &mut RegexSetBuilder {
            self.builder.dot_matches_new_line(yes);
            self
        }