fn find_overlapping_fwd_imp(
    dfa: &DFA,
    cache: &mut Cache,
    input: &Input<'_>,
    pre: Option<&'_ Prefilter>,
    state: &mut OverlappingState,
) -> Result<(), MatchError> {
    // See 'prefilter_restart' docs for explanation.
    let universal_start = dfa.get_nfa().look_set_prefix_any().is_empty();
    let mut sid = match state.id {
        None => {
            state.at = input.start();
            init_fwd(dfa, cache, input)?
        }
        Some(sid) => {
            if let Some(match_index) = state.next_match_index {
                let match_len = dfa.match_len(cache, sid);
                if match_index < match_len {
                    state.next_match_index = Some(match_index + 1);
                    let pattern = dfa.match_pattern(cache, sid, match_index);
                    state.mat = Some(HalfMatch::new(pattern, state.at));
                    return Ok(());
                }
            }
            // Once we've reported all matches at a given position, we need to
            // advance the search to the next position.
            state.at += 1;
            if state.at > input.end() {
                return Ok(());
            }
            sid
        }
    };

    // NOTE: We don't optimize the crap out of this routine primarily because
    // it seems like most overlapping searches will have higher match counts,
    // and thus, throughput is perhaps not as important. But if you have a use
    // case for something faster, feel free to file an issue.
    cache.search_start(state.at);
    while state.at < input.end() {
        sid = dfa
            .next_state(cache, sid, input.haystack()[state.at])
            .map_err(|_| gave_up(state.at))?;
        if sid.is_tagged() {
            state.id = Some(sid);
            if sid.is_start() {
                if let Some(ref pre) = pre {
                    let span = Span::from(state.at..input.end());
                    match pre.find(input.haystack(), span) {
                        None => return Ok(()),
                        Some(ref span) => {
                            if span.start > state.at {
                                state.at = span.start;
                                if !universal_start {
                                    sid = prefilter_restart(
                                        dfa, cache, &input, state.at,
                                    )?;
                                }
                                continue;
                            }
                        }
                    }
                }
            } else if sid.is_match() {
                state.next_match_index = Some(1);
                let pattern = dfa.match_pattern(cache, sid, 0);
                state.mat = Some(HalfMatch::new(pattern, state.at));
                cache.search_finish(state.at);
                return Ok(());
            } else if sid.is_dead() {
                cache.search_finish(state.at);
                return Ok(());
            } else if sid.is_quit() {
                cache.search_finish(state.at);
                return Err(MatchError::quit(
                    input.haystack()[state.at],
                    state.at,
                ));
            } else {
                debug_assert!(sid.is_unknown());
                unreachable!("sid being unknown is a bug");
            }
        }
        state.at += 1;
        cache.search_update(state.at);
    }

    let result = eoi_fwd(dfa, cache, input, &mut sid, &mut state.mat);
    state.id = Some(sid);
    if state.mat.is_some() {
        // '1' is always correct here since if we get to this point, this
        // always corresponds to the first (index '0') match discovered at
        // this position. So the next match to report at this position (if
        // it exists) is at index '1'.
        state.next_match_index = Some(1);
    }
    cache.search_finish(input.end());
    result
}