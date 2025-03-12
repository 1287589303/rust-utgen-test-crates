pub fn minimize(mut self, yes: bool) -> Config {
        self.minimize = Some(yes);
        self
    }