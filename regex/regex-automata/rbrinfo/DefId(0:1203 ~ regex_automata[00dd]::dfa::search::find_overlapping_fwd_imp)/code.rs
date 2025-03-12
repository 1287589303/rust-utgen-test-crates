fn find_overlapping_fwd_imp<A: Automaton + ?Sized>(
    dfa: &A,
    input: &Input<'_>,
    pre: Option<&'_ Prefilter>,
    state: &mut OverlappingState,
) -> Result<(), MatchError> {
    // See 'prefilter_restart' docs for explanation.
    let universal_start = dfa.universal_start_state(Anchored::No).is_some();
    let mut sid = match state.id {
        None => {
            state.at = input.start();
            init_fwd(dfa, input)?
        }
        Some(sid) => {
            if let Some(match_index) = state.next_match_index {
                let match_len = dfa.match_len(sid);
                if match_index < match_len {
                    state.next_match_index = Some(match_index + 1);
                    let pattern = dfa.match_pattern(sid, match_index);
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
    // it seems like most find_overlapping searches will have higher match
    // counts, and thus, throughput is perhaps not as important. But if you
    // have a use case for something faster, feel free to file an issue.
    while state.at < input.end() {
        sid = dfa.next_state(sid, input.haystack()[state.at]);
        if dfa.is_special_state(sid) {
            state.id = Some(sid);
            if dfa.is_start_state(sid) {
                if let Some(ref pre) = pre {
                    let span = Span::from(state.at..input.end());
                    match pre.find(input.haystack(), span) {
                        None => return Ok(()),
                        Some(ref span) => {
                            if span.start > state.at {
                                state.at = span.start;
                                if !universal_start {
                                    sid = prefilter_restart(
                                        dfa, &input, state.at,
                                    )?;
                                }
                                continue;
                            }
                        }
                    }
                } else if dfa.is_accel_state(sid) {
                    let needles = dfa.accelerator(sid);
                    state.at = accel::find_fwd(
                        needles,
                        input.haystack(),
                        state.at + 1,
                    )
                    .unwrap_or(input.end());
                    continue;
                }
            } else if dfa.is_match_state(sid) {
                state.next_match_index = Some(1);
                let pattern = dfa.match_pattern(sid, 0);
                state.mat = Some(HalfMatch::new(pattern, state.at));
                return Ok(());
            } else if dfa.is_accel_state(sid) {
                let needs = dfa.accelerator(sid);
                // If the accelerator returns nothing, why don't we quit the
                // search? Well, if the accelerator doesn't find anything, that
                // doesn't mean we don't have a match. It just means that we
                // can't leave the current state given one of the 255 possible
                // byte values. However, there might be an EOI transition. So
                // we set 'at' to the end of the haystack, which will cause
                // this loop to stop and fall down into the EOI transition.
                state.at =
                    accel::find_fwd(needs, input.haystack(), state.at + 1)
                        .unwrap_or(input.end());
                continue;
            } else if dfa.is_dead_state(sid) {
                return Ok(());
            } else {
                return Err(MatchError::quit(
                    input.haystack()[state.at],
                    state.at,
                ));
            }
        }
        state.at += 1;
    }

    let result = eoi_fwd(dfa, input, &mut sid, &mut state.mat);
    state.id = Some(sid);
    if state.mat.is_some() {
        // '1' is always correct here since if we get to this point, this
        // always corresponds to the first (index '0') match discovered at
        // this position. So the next match to report at this position (if
        // it exists) is at index '1'.
        state.next_match_index = Some(1);
    }
    result
}