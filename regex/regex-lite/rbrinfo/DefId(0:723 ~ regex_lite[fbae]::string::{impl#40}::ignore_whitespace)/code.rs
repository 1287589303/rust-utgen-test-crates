pub fn ignore_whitespace(&mut self, yes: bool) -> &mut RegexBuilder {
        self.hir_config.flags.ignore_whitespace = yes;
        self
    }