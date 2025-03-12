pub(crate) fn new(
        info: &RegexInfo,
        nfarev: &NFA,
    ) -> Option<ReverseHybridEngine> {
        #[cfg(feature = "hybrid")]
        {
            if !info.config().get_hybrid() {
                return None;
            }
            // Since we only use this for reverse searches, we can hard-code
            // a number of things like match semantics, prefilters, starts
            // for each pattern and so on.
            let dfa_config = hybrid::dfa::Config::new()
                .match_kind(MatchKind::All)
                .prefilter(None)
                .starts_for_each_pattern(false)
                .byte_classes(info.config().get_byte_classes())
                .unicode_word_boundary(true)
                .specialize_start_states(false)
                .cache_capacity(info.config().get_hybrid_cache_capacity())
                .skip_cache_capacity_check(false)
                .minimum_cache_clear_count(Some(3))
                .minimum_bytes_per_state(Some(10));
            let result = hybrid::dfa::Builder::new()
                .configure(dfa_config)
                .build_from_nfa(nfarev.clone());
            let rev = match result {
                Ok(rev) => rev,
                Err(_err) => {
                    debug!("lazy reverse DFA failed to build: {}", _err);
                    return None;
                }
            };
            debug!("lazy reverse DFA built");
            Some(ReverseHybridEngine(rev))
        }
        #[cfg(not(feature = "hybrid"))]
        {
            None
        }
    }