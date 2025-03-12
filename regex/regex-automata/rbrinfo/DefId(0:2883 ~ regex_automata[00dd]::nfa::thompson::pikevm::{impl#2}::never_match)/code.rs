pub fn never_match() -> Result<PikeVM, BuildError> {
        let nfa = thompson::NFA::never_match();
        PikeVM::new_from_nfa(nfa)
    }