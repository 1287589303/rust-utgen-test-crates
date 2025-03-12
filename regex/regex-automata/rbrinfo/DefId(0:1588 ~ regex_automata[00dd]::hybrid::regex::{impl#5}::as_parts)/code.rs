pub fn as_parts(&self) -> (&dfa::Cache, &dfa::Cache) {
        (&self.forward, &self.reverse)
    }