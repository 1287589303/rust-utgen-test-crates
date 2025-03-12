pub fn use_idna_2008_rules(mut self, value: bool) -> Self {
        assert!(!value, "IDNA 2008 rules are no longer supported");
        self
    }