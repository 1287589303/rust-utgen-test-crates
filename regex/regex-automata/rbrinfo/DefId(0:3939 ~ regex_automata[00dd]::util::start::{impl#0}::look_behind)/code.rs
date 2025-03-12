pub fn look_behind(mut self, byte: Option<u8>) -> Config {
        self.look_behind = byte;
        self
    }