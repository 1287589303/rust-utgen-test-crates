pub fn new_from_nfa(nfa: NFA) -> Result<DFA, BuildError> {
        DFA::builder().build_from_nfa(nfa)
    }