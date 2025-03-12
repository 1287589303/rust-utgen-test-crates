pub(crate) fn new(info: &RegexInfo, nfarev: &NFA) -> ReverseDFA {
        ReverseDFA(ReverseDFAEngine::new(info, nfarev))
    }