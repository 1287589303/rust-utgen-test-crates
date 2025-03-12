pub(crate) fn new(
        info: &RegexInfo,
        pre: Option<Prefilter>,
        nfa: &NFA,
        nfarev: &NFA,
    ) -> Option<HybridEngine> {
        #[cfg(feature = "hybrid")]
        {
            if !info.config().get_hybrid() {
                return None;
            }
            let dfa_config = hybrid::dfa::Config::new()
                .match_kind(info.config().get_match_kind())
                .prefilter(pre.clone())
                // Enabling this is necessary for ensuring we can service any
                // kind of 'Input' search without error. For the lazy DFA,
                // this is not particularly costly, since the start states are
                // generated lazily.
                .starts_for_each_pattern(true)
                .byte_classes(info.config().get_byte_classes())
                .unicode_word_boundary(true)
                .specialize_start_states(pre.is_some())
                .cache_capacity(info.config().get_hybrid_cache_capacity())
                // This makes it possible for building a lazy DFA to
                // fail even though the NFA has already been built. Namely,
                // if the cache capacity is too small to fit some minimum
                // number of states (which is small, like 4 or 5), then the
                // DFA will refuse to build.
                //
                // We shouldn't enable this to make building always work, since
                // this could cause the allocation of a cache bigger than the
                // provided capacity amount.
                //
                // This is effectively the only reason why building a lazy DFA
                // could fail. If it does, then we simply suppress the error
                // and return None.
                .skip_cache_capacity_check(false)
                // This and enabling heuristic Unicode word boundary support
                // above make it so the lazy DFA can quit at match time.
                .minimum_cache_clear_count(Some(3))
                .minimum_bytes_per_state(Some(10));
            let result = hybrid::dfa::Builder::new()
                .configure(dfa_config.clone())
                .build_from_nfa(nfa.clone());
            let fwd = match result {
                Ok(fwd) => fwd,
                Err(_err) => {
                    debug!("forward lazy DFA failed to build: {}", _err);
                    return None;
                }
            };
            let result = hybrid::dfa::Builder::new()
                .configure(
                    dfa_config
                        .clone()
                        .match_kind(MatchKind::All)
                        .prefilter(None)
                        .specialize_start_states(false),
                )
                .build_from_nfa(nfarev.clone());
            let rev = match result {
                Ok(rev) => rev,
                Err(_err) => {
                    debug!("reverse lazy DFA failed to build: {}", _err);
                    return None;
                }
            };
            let engine =
                hybrid::regex::Builder::new().build_from_dfas(fwd, rev);
            debug!("lazy DFA built");
            Some(HybridEngine(engine))
        }
        #[cfg(not(feature = "hybrid"))]
        {
            None
        }
    }