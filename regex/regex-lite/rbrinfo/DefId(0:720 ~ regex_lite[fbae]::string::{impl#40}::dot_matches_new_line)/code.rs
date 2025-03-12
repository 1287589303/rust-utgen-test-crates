pub fn dot_matches_new_line(&mut self, yes: bool) -> &mut RegexBuilder {
        self.hir_config.flags.dot_matches_new_line = yes;
        self
    }