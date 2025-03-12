pub fn quit(&mut self, set: ByteSet) -> &mut Config {
        self.quit = set;
        self
    }