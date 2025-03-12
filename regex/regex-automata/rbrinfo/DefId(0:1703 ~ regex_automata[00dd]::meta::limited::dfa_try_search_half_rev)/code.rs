pub(crate) fn dfa_try_search_half_rev(
    dfa: &crate::dfa::dense::DFA<alloc::vec::Vec<u32>>,
    input: &Input<'_>,
    min_start: usize,
) -> Result<Option<HalfMatch>, RetryError> {
    use crate::dfa::Automaton;

    let mut mat = None;
    let mut sid = dfa.start_state_reverse(input)?;
    if input.start() == input.end() {
        dfa_eoi_rev(dfa, input, &mut sid, &mut mat)?;
        return Ok(mat);
    }
    let mut at = input.end() - 1;
    loop {
        sid = dfa.next_state(sid, input.haystack()[at]);
        if dfa.is_special_state(sid) {
            if dfa.is_match_state(sid) {
                let pattern = dfa.match_pattern(sid, 0);
                // Since reverse searches report the beginning of a
                // match and the beginning is inclusive (not exclusive
                // like the end of a match), we add 1 to make it
                // inclusive.
                mat = Some(HalfMatch::new(pattern, at + 1));
            } else if dfa.is_dead_state(sid) {
                return Ok(mat);
            } else if dfa.is_quit_state(sid) {
                return Err(MatchError::quit(input.haystack()[at], at).into());
            }
        }
        if at == input.start() {
            break;
        }
        at -= 1;
        if at < min_start {
            trace!(
                "reached position {} which is before the previous literal \
				 match, quitting to avoid quadratic behavior",
                at,
            );
            return Err(RetryError::Quadratic(RetryQuadraticError::new()));
        }
    }
    let was_dead = dfa.is_dead_state(sid);
    dfa_eoi_rev(dfa, input, &mut sid, &mut mat)?;
    // If we reach the beginning of the search and we could otherwise still
    // potentially keep matching if there was more to match, then we actually
    // return an error to indicate giving up on this optimization. Why? Because
    // we can't prove that the real match begins at where we would report it.
    //
    // This only happens when all of the following are true:
    //
    // 1) We reach the starting point of our search span.
    // 2) The match we found is before the starting point.
    // 3) The FSM reports we could possibly find a longer match.
    //
    // We need (1) because otherwise the search stopped before the starting
    // point and there is no possible way to find a more leftmost position.
    //
    // We need (2) because if the match found has an offset equal to the minimum
    // possible offset, then there is no possible more leftmost match.
    //
    // We need (3) because if the FSM couldn't continue anyway (i.e., it's in
    // a dead state), then we know we couldn't find anything more leftmost
    // than what we have. (We have to check the state we were in prior to the
    // EOI transition since the EOI transition will usually bring us to a dead
    // state by virtue of it represents the end-of-input.)
    if at == input.start()
        && mat.map_or(false, |m| m.offset() > input.start())
        && !was_dead
    {
        trace!(
            "reached beginning of search at offset {} without hitting \
             a dead state, quitting to avoid potential false positive match",
            at,
        );
        return Err(RetryError::Quadratic(RetryQuadraticError::new()));
    }
    Ok(mat)
}