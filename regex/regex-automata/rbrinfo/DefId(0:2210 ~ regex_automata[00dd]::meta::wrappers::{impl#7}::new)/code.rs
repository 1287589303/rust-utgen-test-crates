pub(crate) fn new(info: &RegexInfo, nfa: &NFA) -> Option<OnePassEngine> {
        #[cfg(feature = "dfa-onepass")]
        {
            if !info.config().get_onepass() {
                return None;
            }
            // In order to even attempt building a one-pass DFA, we require
            // that we either have at least one explicit capturing group or
            // there's a Unicode word boundary somewhere. If we don't have
            // either of these things, then the lazy DFA will almost certainly
            // be useable and be much faster. The only case where it might
            // not is if the lazy DFA isn't utilizing its cache effectively,
            // but in those cases, the underlying regex is almost certainly
            // not one-pass or is too big to fit within the current one-pass
            // implementation limits.
            if info.props_union().explicit_captures_len() == 0
                && !info.props_union().look_set().contains_word_unicode()
            {
                debug!("not building OnePass because it isn't worth it");
                return None;
            }
            let onepass_config = onepass::Config::new()
                .match_kind(info.config().get_match_kind())
                // Like for the lazy DFA, we unconditionally enable this
                // because it doesn't cost much and makes the API more
                // flexible.
                .starts_for_each_pattern(true)
                .byte_classes(info.config().get_byte_classes())
                .size_limit(info.config().get_onepass_size_limit());
            let result = onepass::Builder::new()
                .configure(onepass_config)
                .build_from_nfa(nfa.clone());
            let engine = match result {
                Ok(engine) => engine,
                Err(_err) => {
                    debug!("OnePass failed to build: {}", _err);
                    return None;
                }
            };
            debug!("OnePass built, {} bytes", engine.memory_usage());
            Some(OnePassEngine(engine))
        }
        #[cfg(not(feature = "dfa-onepass"))]
        {
            None
        }
    }