pub(crate) fn dfa_try_search_half_fwd(
    dfa: &crate::dfa::dense::DFA<alloc::vec::Vec<u32>>,
    input: &Input<'_>,
) -> Result<Result<HalfMatch, usize>, RetryFailError> {
    use crate::dfa::{accel, Automaton};

    let mut mat = None;
    let mut sid = dfa.start_state_forward(input)?;
    let mut at = input.start();
    while at < input.end() {
        sid = dfa.next_state(sid, input.haystack()[at]);
        if dfa.is_special_state(sid) {
            if dfa.is_match_state(sid) {
                let pattern = dfa.match_pattern(sid, 0);
                mat = Some(HalfMatch::new(pattern, at));
                if input.get_earliest() {
                    return Ok(mat.ok_or(at));
                }
                if dfa.is_accel_state(sid) {
                    let needs = dfa.accelerator(sid);
                    at = accel::find_fwd(needs, input.haystack(), at)
                        .unwrap_or(input.end());
                    continue;
                }
            } else if dfa.is_accel_state(sid) {
                let needs = dfa.accelerator(sid);
                at = accel::find_fwd(needs, input.haystack(), at)
                    .unwrap_or(input.end());
                continue;
            } else if dfa.is_dead_state(sid) {
                return Ok(mat.ok_or(at));
            } else if dfa.is_quit_state(sid) {
                return Err(MatchError::quit(input.haystack()[at], at).into());
            } else {
                // Ideally we wouldn't use a DFA that specialized start states
                // and thus 'is_start_state()' could never be true here, but in
                // practice we reuse the DFA created for the full regex which
                // will specialize start states whenever there is a prefilter.
                debug_assert!(dfa.is_start_state(sid));
            }
        }
        at += 1;
    }
    dfa_eoi_fwd(dfa, input, &mut sid, &mut mat)?;
    Ok(mat.ok_or(at))
}