pub fn always_match() -> Result<OwnedDFA, BuildError> {
        let nfa = thompson::NFA::always_match();
        Builder::new().build_from_nfa(&nfa)
    }