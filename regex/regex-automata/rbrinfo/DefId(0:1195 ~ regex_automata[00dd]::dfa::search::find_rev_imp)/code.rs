fn find_rev_imp<A: Automaton + ?Sized>(
    dfa: &A,
    input: &Input<'_>,
    earliest: bool,
) -> Result<Option<HalfMatch>, MatchError> {
    let mut mat = None;
    let mut sid = init_rev(dfa, input)?;
    // In reverse search, the loop below can't handle the case of searching an
    // empty slice. Ideally we could write something congruent to the forward
    // search, i.e., 'while at >= start', but 'start' might be 0. Since we use
    // an unsigned offset, 'at >= 0' is trivially always true. We could avoid
    // this extra case handling by using a signed offset, but Rust makes it
    // annoying to do. So... We just handle the empty case separately.
    if input.start() == input.end() {
        eoi_rev(dfa, input, &mut sid, &mut mat)?;
        return Ok(mat);
    }

    let mut at = input.end() - 1;
    macro_rules! next_unchecked {
        ($sid:expr, $at:expr) => {{
            let byte = *input.haystack().get_unchecked($at);
            dfa.next_state_unchecked($sid, byte)
        }};
    }
    loop {
        // SAFETY: See comments in 'find_fwd' for a safety argument.
        let mut prev_sid;
        while at >= input.start() {
            prev_sid = unsafe { next_unchecked!(sid, at) };
            if dfa.is_special_state(prev_sid)
                || at <= input.start().saturating_add(3)
            {
                core::mem::swap(&mut prev_sid, &mut sid);
                break;
            }
            at -= 1;

            sid = unsafe { next_unchecked!(prev_sid, at) };
            if dfa.is_special_state(sid) {
                break;
            }
            at -= 1;

            prev_sid = unsafe { next_unchecked!(sid, at) };
            if dfa.is_special_state(prev_sid) {
                core::mem::swap(&mut prev_sid, &mut sid);
                break;
            }
            at -= 1;

            sid = unsafe { next_unchecked!(prev_sid, at) };
            if dfa.is_special_state(sid) {
                break;
            }
            at -= 1;
        }
        if dfa.is_special_state(sid) {
            if dfa.is_start_state(sid) {
                if dfa.is_accel_state(sid) {
                    let needles = dfa.accelerator(sid);
                    at = accel::find_rev(needles, input.haystack(), at)
                        .map(|i| i + 1)
                        .unwrap_or(input.start());
                }
            } else if dfa.is_match_state(sid) {
                let pattern = dfa.match_pattern(sid, 0);
                // Since reverse searches report the beginning of a match
                // and the beginning is inclusive (not exclusive like the
                // end of a match), we add 1 to make it inclusive.
                mat = Some(HalfMatch::new(pattern, at + 1));
                if earliest {
                    return Ok(mat);
                }
                if dfa.is_accel_state(sid) {
                    let needles = dfa.accelerator(sid);
                    at = accel::find_rev(needles, input.haystack(), at)
                        .map(|i| i + 1)
                        .unwrap_or(input.start());
                }
            } else if dfa.is_accel_state(sid) {
                let needles = dfa.accelerator(sid);
                // If the accelerator returns nothing, why don't we quit the
                // search? Well, if the accelerator doesn't find anything, that
                // doesn't mean we don't have a match. It just means that we
                // can't leave the current state given one of the 255 possible
                // byte values. However, there might be an EOI transition. So
                // we set 'at' to the end of the haystack, which will cause
                // this loop to stop and fall down into the EOI transition.
                at = accel::find_rev(needles, input.haystack(), at)
                    .map(|i| i + 1)
                    .unwrap_or(input.start());
            } else if dfa.is_dead_state(sid) {
                return Ok(mat);
            } else {
                return Err(MatchError::quit(input.haystack()[at], at));
            }
        }
        if at == input.start() {
            break;
        }
        at -= 1;
    }
    eoi_rev(dfa, input, &mut sid, &mut mat)?;
    Ok(mat)
}