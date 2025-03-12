pub(crate) fn new(info: &RegexInfo, nfa: &NFA) -> OnePass {
        OnePass(OnePassEngine::new(info, nfa))
    }