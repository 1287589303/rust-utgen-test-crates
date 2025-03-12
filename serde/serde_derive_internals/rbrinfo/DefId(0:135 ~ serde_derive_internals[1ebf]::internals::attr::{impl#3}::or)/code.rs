pub fn or(self, other_rules: Self) -> Self {
        Self {
            serialize: self.serialize.or(other_rules.serialize),
            deserialize: self.deserialize.or(other_rules.deserialize),
        }
    }