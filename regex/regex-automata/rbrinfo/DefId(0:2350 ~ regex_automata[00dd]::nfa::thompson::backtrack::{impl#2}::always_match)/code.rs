pub fn always_match() -> Result<BoundedBacktracker, BuildError> {
        let nfa = thompson::NFA::always_match();
        BoundedBacktracker::new_from_nfa(nfa)
    }