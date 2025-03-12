pub fn to_sparse(&self) -> Result<sparse::DFA<Vec<u8>>, BuildError> {
        sparse::DFA::from_dense(self)
    }