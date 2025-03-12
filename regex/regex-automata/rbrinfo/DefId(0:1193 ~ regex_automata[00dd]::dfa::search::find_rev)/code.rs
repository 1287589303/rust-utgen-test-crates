pub fn find_rev<A: Automaton + ?Sized>(
    dfa: &A,
    input: &Input<'_>,
) -> Result<Option<HalfMatch>, MatchError> {
    if input.is_done() {
        return Ok(None);
    }
    if input.get_earliest() {
        find_rev_imp(dfa, input, true)
    } else {
        find_rev_imp(dfa, input, false)
    }
}