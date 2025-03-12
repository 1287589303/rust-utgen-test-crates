pub fn multi_line(&mut self, yes: bool) -> &mut RegexBuilder {
        self.hir_config.flags.multi_line = yes;
        self
    }