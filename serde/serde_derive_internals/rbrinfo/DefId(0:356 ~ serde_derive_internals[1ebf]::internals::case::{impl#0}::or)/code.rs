pub fn or(self, rule_b: Self) -> Self {
        match self {
            None => rule_b,
            _ => self,
        }
    }