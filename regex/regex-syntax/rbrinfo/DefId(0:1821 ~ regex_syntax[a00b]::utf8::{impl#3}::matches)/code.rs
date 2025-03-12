pub fn matches(&self, b: u8) -> bool {
        self.start <= b && b <= self.end
    }