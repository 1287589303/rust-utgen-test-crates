pub fn build_sparse(
        &self,
        pattern: &str,
    ) -> Result<Regex<sparse::DFA<Vec<u8>>>, BuildError> {
        self.build_many_sparse(&[pattern])
    }