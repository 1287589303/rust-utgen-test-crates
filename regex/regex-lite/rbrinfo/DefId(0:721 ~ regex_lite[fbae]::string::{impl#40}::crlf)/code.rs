pub fn crlf(&mut self, yes: bool) -> &mut RegexBuilder {
        self.hir_config.flags.crlf = yes;
        self
    }