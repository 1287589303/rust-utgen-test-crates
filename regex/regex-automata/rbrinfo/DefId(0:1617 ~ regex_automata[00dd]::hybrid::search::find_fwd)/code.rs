pub(crate) fn find_fwd(
    dfa: &DFA,
    cache: &mut Cache,
    input: &Input<'_>,
) -> Result<Option<HalfMatch>, MatchError> {
    if input.is_done() {
        return Ok(None);
    }
    let pre = if input.get_anchored().is_anchored() {
        None
    } else {
        dfa.get_config().get_prefilter()
    };
    // So what we do here is specialize four different versions of 'find_fwd':
    // one for each of the combinations for 'has prefilter' and 'is earliest
    // search'. The reason for doing this is that both of these things require
    // branches and special handling in some code that can be very hot,
    // and shaving off as much as we can when we don't need it tends to be
    // beneficial in ad hoc benchmarks. To see these differences, you often
    // need a query with a high match count. In other words, specializing these
    // four routines *tends* to help latency more than throughput.
    if pre.is_some() {
        if input.get_earliest() {
            find_fwd_imp(dfa, cache, input, pre, true)
        } else {
            find_fwd_imp(dfa, cache, input, pre, false)
        }
    } else {
        if input.get_earliest() {
            find_fwd_imp(dfa, cache, input, None, true)
        } else {
            find_fwd_imp(dfa, cache, input, None, false)
        }
    }
}