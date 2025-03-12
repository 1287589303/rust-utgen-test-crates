pub fn swap_greed(&mut self, yes: bool) -> &mut RegexBuilder {
        self.hir_config.flags.swap_greed = yes;
        self
    }