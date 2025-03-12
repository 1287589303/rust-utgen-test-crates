pub fn never_match() -> Result<BoundedBacktracker, BuildError> {
        let nfa = thompson::NFA::never_match();
        BoundedBacktracker::new_from_nfa(nfa)
    }