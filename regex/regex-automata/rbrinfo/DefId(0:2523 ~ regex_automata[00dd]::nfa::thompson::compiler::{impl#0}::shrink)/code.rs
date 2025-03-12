pub fn shrink(mut self, yes: bool) -> Config {
        self.shrink = Some(yes);
        self
    }