pub fn new_from_nfa(nfa: NFA) -> Result<BoundedBacktracker, BuildError> {
        BoundedBacktracker::builder().build_from_nfa(nfa)
    }