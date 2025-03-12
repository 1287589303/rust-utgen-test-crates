pub(crate) fn new(
        info: &RegexInfo,
        pre: Option<Prefilter>,
        nfa: &NFA,
        nfarev: &NFA,
    ) -> Option<DFAEngine> {
        #[cfg(feature = "dfa-build")]
        {
            if !info.config().get_dfa() {
                return None;
            }
            // If our NFA is anything but small, don't even bother with a DFA.
            if let Some(state_limit) = info.config().get_dfa_state_limit() {
                if nfa.states().len() > state_limit {
                    debug!(
                        "skipping full DFA because NFA has {} states, \
                         which exceeds the heuristic limit of {}",
                        nfa.states().len(),
                        state_limit,
                    );
                    return None;
                }
            }
            // We cut the size limit in four because the total heap used by
            // DFA construction is determinization aux memory and the DFA
            // itself, and those things are configured independently in the
            // lower level DFA builder API. And then split that in two because
            // of forward and reverse DFAs.
            let size_limit = info.config().get_dfa_size_limit().map(|n| n / 4);
            let dfa_config = dfa::dense::Config::new()
                .match_kind(info.config().get_match_kind())
                .prefilter(pre.clone())
                // Enabling this is necessary for ensuring we can service any
                // kind of 'Input' search without error. For the full DFA, this
                // can be quite costly. But since we have such a small bound
                // on the size of the DFA, in practice, any multl-regexes are
                // probably going to blow the limit anyway.
                .starts_for_each_pattern(true)
                .byte_classes(info.config().get_byte_classes())
                .unicode_word_boundary(true)
                .specialize_start_states(pre.is_some())
                .determinize_size_limit(size_limit)
                .dfa_size_limit(size_limit);
            let result = dfa::dense::Builder::new()
                .configure(dfa_config.clone())
                .build_from_nfa(&nfa);
            let fwd = match result {
                Ok(fwd) => fwd,
                Err(_err) => {
                    debug!("forward full DFA failed to build: {}", _err);
                    return None;
                }
            };
            let result = dfa::dense::Builder::new()
                .configure(
                    dfa_config
                        .clone()
                        // We never need unanchored reverse searches, so
                        // there's no point in building it into the DFA, which
                        // WILL take more space. (This isn't done for the lazy
                        // DFA because the DFA is, well, lazy. It doesn't pay
                        // the cost for supporting unanchored searches unless
                        // you actually do an unanchored search, which we
                        // don't.)
                        .start_kind(dfa::StartKind::Anchored)
                        .match_kind(MatchKind::All)
                        .prefilter(None)
                        .specialize_start_states(false),
                )
                .build_from_nfa(&nfarev);
            let rev = match result {
                Ok(rev) => rev,
                Err(_err) => {
                    debug!("reverse full DFA failed to build: {}", _err);
                    return None;
                }
            };
            let engine = dfa::regex::Builder::new().build_from_dfas(fwd, rev);
            debug!(
                "fully compiled forward and reverse DFAs built, {} bytes",
                engine.forward().memory_usage()
                    + engine.reverse().memory_usage(),
            );
            Some(DFAEngine(engine))
        }
        #[cfg(not(feature = "dfa-build"))]
        {
            None
        }
    }