pub fn as_str(&self) -> &str {
        &self.pikevm.nfa().pattern()
    }