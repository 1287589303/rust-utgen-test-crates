pub fn always_match() -> Result<PikeVM, BuildError> {
        let nfa = thompson::NFA::always_match();
        PikeVM::new_from_nfa(nfa)
    }