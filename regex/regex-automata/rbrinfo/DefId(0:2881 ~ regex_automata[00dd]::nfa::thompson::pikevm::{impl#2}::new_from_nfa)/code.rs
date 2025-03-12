pub fn new_from_nfa(nfa: NFA) -> Result<PikeVM, BuildError> {
        PikeVM::builder().build_from_nfa(nfa)
    }