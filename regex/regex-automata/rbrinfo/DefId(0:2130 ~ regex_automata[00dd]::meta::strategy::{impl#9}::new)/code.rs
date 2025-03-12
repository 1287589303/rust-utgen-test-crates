fn new(core: Core, hirs: &[&Hir]) -> Result<ReverseInner, Core> {
        if !core.info.config().get_auto_prefilter() {
            debug!(
                "skipping reverse inner optimization because \
                 automatic prefilters are disabled"
            );
            return Err(core);
        }
        // Currently we hard-code the assumption of leftmost-first match
        // semantics. This isn't a huge deal because 'all' semantics tend to
        // only be used for forward overlapping searches with multiple regexes,
        // and this optimization only supports a single pattern at the moment.
        if core.info.config().get_match_kind() != MatchKind::LeftmostFirst {
            debug!(
                "skipping reverse inner optimization because \
				 match kind is {:?} but this only supports leftmost-first",
                core.info.config().get_match_kind(),
            );
            return Err(core);
        }
        // It's likely that a reverse inner scan has too much overhead for it
        // to be worth it when the regex is anchored at the start. It is
        // possible for it to be quite a bit faster if the initial literal
        // scan fails to detect a match, in which case, we can say "no match"
        // very quickly. But this could be undesirable, e.g., scanning too far
        // or when the literal scan matches. If it matches, then confirming the
        // match requires a reverse scan followed by a forward scan to confirm
        // or reject, which is a fair bit of work.
        //
        // Note that the caller can still request an anchored search even
        // when the regex isn't anchored. We detect that case in the search
        // routines below and just fallback to the core engine. Currently this
        // optimization assumes all searches are unanchored, so if we do want
        // to enable this optimization for anchored searches, it will need a
        // little work to support it.
        if core.info.is_always_anchored_start() {
            debug!(
                "skipping reverse inner optimization because \
				 the regex is always anchored at the start",
            );
            return Err(core);
        }
        // Only DFAs can do reverse searches (currently), so we need one of
        // them in order to do this optimization. It's possible (although
        // pretty unlikely) that we have neither and need to give up.
        if !core.hybrid.is_some() && !core.dfa.is_some() {
            debug!(
                "skipping reverse inner optimization because \
				 we don't have a lazy DFA or a full DFA"
            );
            return Err(core);
        }
        if core.pre.as_ref().map_or(false, |p| p.is_fast()) {
            debug!(
                "skipping reverse inner optimization because \
				 we already have a prefilter that we think is fast"
            );
            return Err(core);
        } else if core.pre.is_some() {
            debug!(
                "core engine has a prefix prefilter, but it is \
                 probably not fast, so continuing with attempt to \
                 use reverse inner prefilter"
            );
        }
        let (concat_prefix, preinner) = match reverse_inner::extract(hirs) {
            Some(x) => x,
            // N.B. the 'extract' function emits debug messages explaining
            // why we bailed out here.
            None => return Err(core),
        };
        debug!("building reverse NFA for prefix before inner literal");
        let mut lookm = LookMatcher::new();
        lookm.set_line_terminator(core.info.config().get_line_terminator());
        let thompson_config = thompson::Config::new()
            .reverse(true)
            .utf8(core.info.config().get_utf8_empty())
            .nfa_size_limit(core.info.config().get_nfa_size_limit())
            .shrink(false)
            .which_captures(WhichCaptures::None)
            .look_matcher(lookm);
        let result = thompson::Compiler::new()
            .configure(thompson_config)
            .build_from_hir(&concat_prefix);
        let nfarev = match result {
            Ok(nfarev) => nfarev,
            Err(_err) => {
                debug!(
                    "skipping reverse inner optimization because the \
					 reverse NFA failed to build: {}",
                    _err,
                );
                return Err(core);
            }
        };
        debug!("building reverse DFA for prefix before inner literal");
        let dfa = if !core.info.config().get_dfa() {
            wrappers::ReverseDFA::none()
        } else {
            wrappers::ReverseDFA::new(&core.info, &nfarev)
        };
        let hybrid = if !core.info.config().get_hybrid() {
            wrappers::ReverseHybrid::none()
        } else if dfa.is_some() {
            debug!(
                "skipping lazy DFA for reverse inner optimization \
				 because we have a full DFA"
            );
            wrappers::ReverseHybrid::none()
        } else {
            wrappers::ReverseHybrid::new(&core.info, &nfarev)
        };
        Ok(ReverseInner { core, preinner, nfarev, hybrid, dfa })
    }