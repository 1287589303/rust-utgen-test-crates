pub fn nest_limit(&mut self, limit: u32) -> &mut RegexBuilder {
        self.hir_config.nest_limit = limit;
        self
    }