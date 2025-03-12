pub(crate) fn new(
        info: &RegexInfo,
        pre: Option<Prefilter>,
        nfa: &NFA,
    ) -> Result<Option<BoundedBacktrackerEngine>, BuildError> {
        #[cfg(feature = "nfa-backtrack")]
        {
            if !info.config().get_backtrack()
                || info.config().get_match_kind() != MatchKind::LeftmostFirst
            {
                return Ok(None);
            }
            let backtrack_config = backtrack::Config::new().prefilter(pre);
            let engine = backtrack::Builder::new()
                .configure(backtrack_config)
                .build_from_nfa(nfa.clone())
                .map_err(BuildError::nfa)?;
            debug!(
                "BoundedBacktracker built (max haystack length: {:?})",
                engine.max_haystack_len()
            );
            Ok(Some(BoundedBacktrackerEngine(engine)))
        }
        #[cfg(not(feature = "nfa-backtrack"))]
        {
            Ok(None)
        }
    }