pub(crate) fn dead() -> State {
        StateBuilderEmpty::new().into_matches().into_nfa().to_state()
    }