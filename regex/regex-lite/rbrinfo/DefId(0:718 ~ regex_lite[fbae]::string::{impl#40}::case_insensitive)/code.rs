pub fn case_insensitive(&mut self, yes: bool) -> &mut RegexBuilder {
        self.hir_config.flags.case_insensitive = yes;
        self
    }