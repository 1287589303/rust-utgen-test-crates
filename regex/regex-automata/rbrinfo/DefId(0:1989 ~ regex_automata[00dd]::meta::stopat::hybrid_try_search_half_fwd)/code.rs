pub(crate) fn hybrid_try_search_half_fwd(
    dfa: &crate::hybrid::dfa::DFA,
    cache: &mut crate::hybrid::dfa::Cache,
    input: &Input<'_>,
) -> Result<Result<HalfMatch, usize>, RetryFailError> {
    let mut mat = None;
    let mut sid = dfa.start_state_forward(cache, input)?;
    let mut at = input.start();
    while at < input.end() {
        sid = dfa
            .next_state(cache, sid, input.haystack()[at])
            .map_err(|_| MatchError::gave_up(at))?;
        if sid.is_tagged() {
            if sid.is_match() {
                let pattern = dfa.match_pattern(cache, sid, 0);
                mat = Some(HalfMatch::new(pattern, at));
                if input.get_earliest() {
                    return Ok(mat.ok_or(at));
                }
            } else if sid.is_dead() {
                return Ok(mat.ok_or(at));
            } else if sid.is_quit() {
                return Err(MatchError::quit(input.haystack()[at], at).into());
            } else {
                // We should NEVER get an unknown state ID back from
                // dfa.next_state().
                debug_assert!(!sid.is_unknown());
                // Ideally we wouldn't use a lazy DFA that specialized start
                // states and thus 'sid.is_start()' could never be true here,
                // but in practice we reuse the lazy DFA created for the full
                // regex which will specialize start states whenever there is
                // a prefilter.
                debug_assert!(sid.is_start());
            }
        }
        at += 1;
    }
    hybrid_eoi_fwd(dfa, cache, input, &mut sid, &mut mat)?;
    Ok(mat.ok_or(at))
}