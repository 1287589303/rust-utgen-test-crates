pub fn always_match() -> Result<DFA<Vec<u8>>, BuildError> {
        dense::DFA::always_match()?.to_sparse()
    }