pub fn onepass(self, yes: bool) -> Config {
        Config { onepass: Some(yes), ..self }
    }