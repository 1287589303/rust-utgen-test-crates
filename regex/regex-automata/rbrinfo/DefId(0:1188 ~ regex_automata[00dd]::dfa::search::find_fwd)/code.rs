pub fn find_fwd<A: Automaton + ?Sized>(
    dfa: &A,
    input: &Input<'_>,
) -> Result<Option<HalfMatch>, MatchError> {
    if input.is_done() {
        return Ok(None);
    }
    let pre = if input.get_anchored().is_anchored() {
        None
    } else {
        dfa.get_prefilter()
    };
    // Searching with a pattern ID is always anchored, so we should never use
    // a prefilter.
    if pre.is_some() {
        if input.get_earliest() {
            find_fwd_imp(dfa, input, pre, true)
        } else {
            find_fwd_imp(dfa, input, pre, false)
        }
    } else {
        if input.get_earliest() {
            find_fwd_imp(dfa, input, None, true)
        } else {
            find_fwd_imp(dfa, input, None, false)
        }
    }
}