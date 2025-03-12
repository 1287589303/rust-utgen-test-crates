pub fn size_limit(&mut self, limit: usize) -> &mut RegexBuilder {
        self.nfa_config.size_limit = Some(limit);
        self
    }