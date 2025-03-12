pub(crate) fn find_overlapping_rev<A: Automaton + ?Sized>(
    dfa: &A,
    input: &Input<'_>,
    state: &mut OverlappingState,
) -> Result<(), MatchError> {
    state.mat = None;
    if input.is_done() {
        return Ok(());
    }
    let mut sid = match state.id {
        None => {
            let sid = init_rev(dfa, input)?;
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
                let match_len = dfa.match_len(sid);
                if match_index < match_len {
                    state.next_match_index = Some(match_index + 1);
                    let pattern = dfa.match_pattern(sid, match_index);
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
    while !state.rev_eoi {
        sid = dfa.next_state(sid, input.haystack()[state.at]);
        if dfa.is_special_state(sid) {
            state.id = Some(sid);
            if dfa.is_start_state(sid) {
                if dfa.is_accel_state(sid) {
                    let needles = dfa.accelerator(sid);
                    state.at =
                        accel::find_rev(needles, input.haystack(), state.at)
                            .map(|i| i + 1)
                            .unwrap_or(input.start());
                }
            } else if dfa.is_match_state(sid) {
                state.next_match_index = Some(1);
                let pattern = dfa.match_pattern(sid, 0);
                state.mat = Some(HalfMatch::new(pattern, state.at + 1));
                return Ok(());
            } else if dfa.is_accel_state(sid) {
                let needles = dfa.accelerator(sid);
                // If the accelerator returns nothing, why don't we quit the
                // search? Well, if the accelerator doesn't find anything, that
                // doesn't mean we don't have a match. It just means that we
                // can't leave the current state given one of the 255 possible
                // byte values. However, there might be an EOI transition. So
                // we set 'at' to the end of the haystack, which will cause
                // this loop to stop and fall down into the EOI transition.
                state.at =
                    accel::find_rev(needles, input.haystack(), state.at)
                        .map(|i| i + 1)
                        .unwrap_or(input.start());
            } else if dfa.is_dead_state(sid) {
                return Ok(());
            } else {
                return Err(MatchError::quit(
                    input.haystack()[state.at],
                    state.at,
                ));
            }
        }
        if state.at == input.start() {
            break;
        }
        state.at -= 1;
    }

    let result = eoi_rev(dfa, input, &mut sid, &mut state.mat);
    state.rev_eoi = true;
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