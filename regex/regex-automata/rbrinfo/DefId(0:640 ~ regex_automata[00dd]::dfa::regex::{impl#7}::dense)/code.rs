pub fn dense(&mut self, config: dense::Config) -> &mut Builder {
        self.dfa.configure(config);
        self
    }