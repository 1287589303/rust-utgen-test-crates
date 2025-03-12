pub fn accelerate(mut self, yes: bool) -> Config {
        self.accelerate = Some(yes);
        self
    }