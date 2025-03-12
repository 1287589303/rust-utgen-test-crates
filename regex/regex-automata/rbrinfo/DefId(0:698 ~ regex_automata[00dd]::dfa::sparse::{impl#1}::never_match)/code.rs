pub fn never_match() -> Result<DFA<Vec<u8>>, BuildError> {
        dense::DFA::never_match()?.to_sparse()
    }