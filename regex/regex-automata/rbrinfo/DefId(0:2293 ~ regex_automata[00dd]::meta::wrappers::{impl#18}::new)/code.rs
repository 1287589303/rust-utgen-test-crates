pub(crate) fn new(
        info: &RegexInfo,
        nfarev: &NFA,
    ) -> Option<ReverseDFAEngine> {
        #[cfg(feature = "dfa-build")]
        {
            if !info.config().get_dfa() {
                return None;
            }
            // If our NFA is anything but small, don't even bother with a DFA.
            if let Some(state_limit) = info.config().get_dfa_state_limit() {
                if nfarev.states().len() > state_limit {
                    debug!(
                        "skipping full reverse DFA because NFA has {} states, \
                         which exceeds the heuristic limit of {}",
                        nfarev.states().len(),
                        state_limit,
					);
                    return None;
                }
            }
            // We cut the size limit in two because the total heap used by DFA
            // construction is determinization aux memory and the DFA itself,
            // and those things are configured independently in the lower level
            // DFA builder API.
            let size_limit = info.config().get_dfa_size_limit().map(|n| n / 2);
            // Since we only use this for reverse searches, we can hard-code
            // a number of things like match semantics, prefilters, starts
            // for each pattern and so on. We also disable acceleration since
            // it's incompatible with limited searches (which is the only
            // operation we support for this kind of engine at the moment).
            let dfa_config = dfa::dense::Config::new()
                .match_kind(MatchKind::All)
                .prefilter(None)
                .accelerate(false)
                .start_kind(dfa::StartKind::Anchored)
                .starts_for_each_pattern(false)
                .byte_classes(info.config().get_byte_classes())
                .unicode_word_boundary(true)
                .specialize_start_states(false)
                .determinize_size_limit(size_limit)
                .dfa_size_limit(size_limit);
            let result = dfa::dense::Builder::new()
                .configure(dfa_config)
                .build_from_nfa(&nfarev);
            let rev = match result {
                Ok(rev) => rev,
                Err(_err) => {
                    debug!("full reverse DFA failed to build: {}", _err);
                    return None;
                }
            };
            debug!(
                "fully compiled reverse DFA built, {} bytes",
                rev.memory_usage()
            );
            Some(ReverseDFAEngine(rev))
        }
        #[cfg(not(feature = "dfa-build"))]
        {
            None
        }
    }