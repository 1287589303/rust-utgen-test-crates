pub(crate) fn new(
        info: &RegexInfo,
        pre: Option<Prefilter>,
        nfa: &NFA,
    ) -> Result<BoundedBacktracker, BuildError> {
        BoundedBacktrackerEngine::new(info, pre, nfa).map(BoundedBacktracker)
    }