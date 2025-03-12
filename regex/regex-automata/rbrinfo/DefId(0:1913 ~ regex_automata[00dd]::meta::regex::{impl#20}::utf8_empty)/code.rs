pub fn utf8_empty(self, yes: bool) -> Config {
        Config { utf8_empty: Some(yes), ..self }
    }