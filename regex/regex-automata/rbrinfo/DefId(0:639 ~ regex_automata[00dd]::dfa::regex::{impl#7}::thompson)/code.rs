pub fn thompson(
        &mut self,
        config: crate::nfa::thompson::Config,
    ) -> &mut Builder {
        self.dfa.thompson(config);
        self
    }