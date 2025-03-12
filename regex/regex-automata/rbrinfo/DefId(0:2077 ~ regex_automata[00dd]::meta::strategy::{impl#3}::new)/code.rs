fn new(
        info: RegexInfo,
        pre: Option<Prefilter>,
        hirs: &[&Hir],
    ) -> Result<Core, BuildError> {
        let mut lookm = LookMatcher::new();
        lookm.set_line_terminator(info.config().get_line_terminator());
        let thompson_config = thompson::Config::new()
            .utf8(info.config().get_utf8_empty())
            .nfa_size_limit(info.config().get_nfa_size_limit())
            .shrink(false)
            .which_captures(info.config().get_which_captures())
            .look_matcher(lookm);
        let nfa = thompson::Compiler::new()
            .configure(thompson_config.clone())
            .build_many_from_hir(hirs)
            .map_err(BuildError::nfa)?;
        // It's possible for the PikeVM or the BB to fail to build, even though
        // at this point, we already have a full NFA in hand. They can fail
        // when a Unicode word boundary is used but where Unicode word boundary
        // support is disabled at compile time, thus making it impossible to
        // match. (Construction can also fail if the NFA was compiled without
        // captures, but we always enable that above.)
        let pikevm = wrappers::PikeVM::new(&info, pre.clone(), &nfa)?;
        let backtrack =
            wrappers::BoundedBacktracker::new(&info, pre.clone(), &nfa)?;
        // The onepass engine can of course fail to build, but we expect it to
        // fail in many cases because it is an optimization that doesn't apply
        // to all regexes. The 'OnePass' wrapper encapsulates this failure (and
        // logs a message if it occurs).
        let onepass = wrappers::OnePass::new(&info, &nfa);
        // We try to encapsulate whether a particular regex engine should be
        // used within each respective wrapper, but the DFAs need a reverse NFA
        // to build itself, and we really do not want to build a reverse NFA if
        // we know we aren't going to use the lazy DFA. So we do a config check
        // up front, which is in practice the only way we won't try to use the
        // DFA.
        let (nfarev, hybrid, dfa) =
            if !info.config().get_hybrid() && !info.config().get_dfa() {
                (None, wrappers::Hybrid::none(), wrappers::DFA::none())
            } else {
                // FIXME: Technically, we don't quite yet KNOW that we need
                // a reverse NFA. It's possible for the DFAs below to both
                // fail to build just based on the forward NFA. In which case,
                // building the reverse NFA was totally wasted work. But...
                // fixing this requires breaking DFA construction apart into
                // two pieces: one for the forward part and another for the
                // reverse part. Quite annoying. Making it worse, when building
                // both DFAs fails, it's quite likely that the NFA is large and
                // that it will take quite some time to build the reverse NFA
                // too. So... it's really probably worth it to do this!
                let nfarev = thompson::Compiler::new()
                    // Currently, reverse NFAs don't support capturing groups,
                    // so we MUST disable them. But even if we didn't have to,
                    // we would, because nothing in this crate does anything
                    // useful with capturing groups in reverse. And of course,
                    // the lazy DFA ignores capturing groups in all cases.
                    .configure(
                        thompson_config
                            .clone()
                            .which_captures(WhichCaptures::None)
                            .reverse(true),
                    )
                    .build_many_from_hir(hirs)
                    .map_err(BuildError::nfa)?;
                let dfa = if !info.config().get_dfa() {
                    wrappers::DFA::none()
                } else {
                    wrappers::DFA::new(&info, pre.clone(), &nfa, &nfarev)
                };
                let hybrid = if !info.config().get_hybrid() {
                    wrappers::Hybrid::none()
                } else if dfa.is_some() {
                    debug!("skipping lazy DFA because we have a full DFA");
                    wrappers::Hybrid::none()
                } else {
                    wrappers::Hybrid::new(&info, pre.clone(), &nfa, &nfarev)
                };
                (Some(nfarev), hybrid, dfa)
            };
        Ok(Core {
            info,
            pre,
            nfa,
            nfarev,
            pikevm,
            backtrack,
            onepass,
            hybrid,
            dfa,
        })
    }