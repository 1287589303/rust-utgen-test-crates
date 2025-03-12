pub(crate) fn find_overlapping_rev(
    dfa: &DFA,
    cache: &mut Cache,
    input: &Input<'_>,
    state: &mut OverlappingState,
) -> Result<(), MatchError> {
    state.mat = None;
    if input.is_done() {
        return Ok(());
    }
    let mut sid = match state.id {
        None => {
            let sid = init_rev(dfa, cache, input)?;
            state.id = Some(sid);
            if input.start() == input.end() {
                state.rev_eoi = true;
            } else {
                state.at = input.end() - 1;
            }
            sid
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
            // Once we've reported all matches at a given position, we need
            // to advance the search to the next position. However, if we've
            // already followed the EOI transition, then we know we're done
            // with the search and there cannot be any more matches to report.
            if state.rev_eoi {
                return Ok(());
            } else if state.at == input.start() {
                // At this point, we should follow the EOI transition. This
                // will cause us the skip the main loop below and fall through
                // to the final 'eoi_rev' transition.
                state.rev_eoi = true;
            } else {
                // We haven't hit the end of the search yet, so move on.
                state.at -= 1;
            }
            sid
        }
    };
    cache.search_start(state.at);
    while !state.rev_eoi {
        sid = dfa
            .next_state(cache, sid, input.haystack()[state.at])
            .map_err(|_| gave_up(state.at))?;
        if sid.is_tagged() {
            state.id = Some(sid);
            if sid.is_start() {
                // do nothing
            } else if sid.is_match() {
                state.next_match_index = Some(1);
                let pattern = dfa.match_pattern(cache, sid, 0);
                state.mat = Some(HalfMatch::new(pattern, state.at + 1));
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
        if state.at == input.start() {
            break;
        }
        state.at -= 1;
        cache.search_update(state.at);
    }

    let result = eoi_rev(dfa, cache, input, &mut sid, &mut state.mat);
    state.rev_eoi = true;
    state.id = Some(sid);
    if state.mat.is_some() {
        // '1' is always correct here since if we get to this point, this
        // always corresponds to the first (index '0') match discovered at
        // this position. So the next match to report at this position (if
        // it exists) is at index '1'.
        state.next_match_index = Some(1);
    }
    cache.search_finish(input.start());
    result
}