fn find_fwd_imp<A: Automaton + ?Sized>(
    dfa: &A,
    input: &Input<'_>,
    pre: Option<&'_ Prefilter>,
    earliest: bool,
) -> Result<Option<HalfMatch>, MatchError> {
    // See 'prefilter_restart' docs for explanation.
    let universal_start = dfa.universal_start_state(Anchored::No).is_some();
    let mut mat = None;
    let mut sid = init_fwd(dfa, input)?;
    let mut at = input.start();
    // This could just be a closure, but then I think it would be unsound
    // because it would need to be safe to invoke. This way, the lack of safety
    // is clearer in the code below.
    macro_rules! next_unchecked {
        ($sid:expr, $at:expr) => {{
            let byte = *input.haystack().get_unchecked($at);
            dfa.next_state_unchecked($sid, byte)
        }};
    }

    if let Some(ref pre) = pre {
        let span = Span::from(at..input.end());
        // If a prefilter doesn't report false positives, then we don't need to
        // touch the DFA at all. However, since all matches include the pattern
        // ID, and the prefilter infrastructure doesn't report pattern IDs, we
        // limit this optimization to cases where there is exactly one pattern.
        // In that case, any match must be the 0th pattern.
        match pre.find(input.haystack(), span) {
            None => return Ok(mat),
            Some(ref span) => {
                at = span.start;
                if !universal_start {
                    sid = prefilter_restart(dfa, &input, at)?;
                }
            }
        }
    }
    while at < input.end() {
        // SAFETY: There are two safety invariants we need to uphold here in
        // the loops below: that 'sid' and 'prev_sid' are valid state IDs
        // for this DFA, and that 'at' is a valid index into 'haystack'.
        // For the former, we rely on the invariant that next_state* and
        // start_state_forward always returns a valid state ID (given a valid
        // state ID in the former case). For the latter safety invariant, we
        // always guard unchecked access with a check that 'at' is less than
        // 'end', where 'end <= haystack.len()'. In the unrolled loop below, we
        // ensure that 'at' is always in bounds.
        //
        // PERF: See a similar comment in src/hybrid/search.rs that justifies
        // this extra work to make the search loop fast. The same reasoning and
        // benchmarks apply here.
        let mut prev_sid;
        while at < input.end() {
            prev_sid = unsafe { next_unchecked!(sid, at) };
            if dfa.is_special_state(prev_sid) || at + 3 >= input.end() {
                core::mem::swap(&mut prev_sid, &mut sid);
                break;
            }
            at += 1;

            sid = unsafe { next_unchecked!(prev_sid, at) };
            if dfa.is_special_state(sid) {
                break;
            }
            at += 1;

            prev_sid = unsafe { next_unchecked!(sid, at) };
            if dfa.is_special_state(prev_sid) {
                core::mem::swap(&mut prev_sid, &mut sid);
                break;
            }
            at += 1;

            sid = unsafe { next_unchecked!(prev_sid, at) };
            if dfa.is_special_state(sid) {
                break;
            }
            at += 1;
        }
        if dfa.is_special_state(sid) {
            if dfa.is_start_state(sid) {
                if let Some(ref pre) = pre {
                    let span = Span::from(at..input.end());
                    match pre.find(input.haystack(), span) {
                        None => return Ok(mat),
                        Some(ref span) => {
                            // We want to skip any update to 'at' below
                            // at the end of this iteration and just
                            // jump immediately back to the next state
                            // transition at the leading position of the
                            // candidate match.
                            //
                            // ... but only if we actually made progress
                            // with our prefilter, otherwise if the start
                            // state has a self-loop, we can get stuck.
                            if span.start > at {
                                at = span.start;
                                if !universal_start {
                                    sid = prefilter_restart(dfa, &input, at)?;
                                }
                                continue;
                            }
                        }
                    }
                } else if dfa.is_accel_state(sid) {
                    let needles = dfa.accelerator(sid);
                    at = accel::find_fwd(needles, input.haystack(), at + 1)
                        .unwrap_or(input.end());
                    continue;
                }
            } else if dfa.is_match_state(sid) {
                let pattern = dfa.match_pattern(sid, 0);
                mat = Some(HalfMatch::new(pattern, at));
                if earliest {
                    return Ok(mat);
                }
                if dfa.is_accel_state(sid) {
                    let needles = dfa.accelerator(sid);
                    at = accel::find_fwd(needles, input.haystack(), at + 1)
                        .unwrap_or(input.end());
                    continue;
                }
            } else if dfa.is_accel_state(sid) {
                let needs = dfa.accelerator(sid);
                at = accel::find_fwd(needs, input.haystack(), at + 1)
                    .unwrap_or(input.end());
                continue;
            } else if dfa.is_dead_state(sid) {
                return Ok(mat);
            } else {
                // It's important that this is a debug_assert, since this can
                // actually be tripped even if DFA::from_bytes succeeds and
                // returns a supposedly valid DFA.
                return Err(MatchError::quit(input.haystack()[at], at));
            }
        }
        at += 1;
    }
    eoi_fwd(dfa, input, &mut sid, &mut mat)?;
    Ok(mat)
}