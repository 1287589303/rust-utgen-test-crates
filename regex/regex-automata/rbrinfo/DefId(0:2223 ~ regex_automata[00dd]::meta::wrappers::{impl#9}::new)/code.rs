pub(crate) fn new(
        info: &RegexInfo,
        pre: Option<Prefilter>,
        nfa: &NFA,
        nfarev: &NFA,
    ) -> Hybrid {
        Hybrid(HybridEngine::new(info, pre, nfa, nfarev))
    }