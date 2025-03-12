pub(crate) fn new(
        info: &RegexInfo,
        pre: Option<Prefilter>,
        nfa: &NFA,
        nfarev: &NFA,
    ) -> DFA {
        DFA(DFAEngine::new(info, pre, nfa, nfarev))
    }