pub fn matches(&self, bytes: &[u8]) -> bool {
        if bytes.len() < self.len() {
            return false;
        }
        for (&b, r) in bytes.iter().zip(self) {
            if !r.matches(b) {
                return false;
            }
        }
        true
    }