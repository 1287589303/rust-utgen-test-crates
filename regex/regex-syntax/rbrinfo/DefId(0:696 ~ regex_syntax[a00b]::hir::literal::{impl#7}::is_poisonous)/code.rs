fn is_poisonous(&self) -> bool {
        self.is_empty() || (self.len() == 1 && rank(self.as_bytes()[0]) >= 250)
    }