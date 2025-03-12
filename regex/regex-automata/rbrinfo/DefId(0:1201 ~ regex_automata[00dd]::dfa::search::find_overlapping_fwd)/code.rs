pub fn find_overlapping_fwd<A: Automaton + ?Sized>(
    dfa: &A,
    input: &Input<'_>,
    state: &mut OverlappingState,
) -> Result<(), MatchError> {
    state.mat = None;
    if input.is_done() {
        return Ok(());
    }
    let pre = if input.get_anchored().is_anchored() {
        None
    } else {
        dfa.get_prefilter()
    };
    if pre.is_some() {
        find_overlapping_fwd_imp(dfa, input, pre, state)
    } else {
        find_overlapping_fwd_imp(dfa, input, None, state)
    }
}