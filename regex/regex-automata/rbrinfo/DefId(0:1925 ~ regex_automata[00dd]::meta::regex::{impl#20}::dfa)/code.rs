pub fn dfa(self, yes: bool) -> Config {
        Config { dfa: Some(yes), ..self }
    }