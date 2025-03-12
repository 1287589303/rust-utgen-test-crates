pub fn specialize_start_states(mut self, yes: bool) -> Config {
        self.specialize_start_states = Some(yes);
        self
    }