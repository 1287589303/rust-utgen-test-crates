pub fn never_match() -> Result<OwnedDFA, BuildError> {
        let nfa = thompson::NFA::never_match();
        Builder::new().build_from_nfa(&nfa)
    }