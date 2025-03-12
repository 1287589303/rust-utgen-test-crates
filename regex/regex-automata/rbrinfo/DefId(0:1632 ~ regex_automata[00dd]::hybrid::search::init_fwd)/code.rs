fn init_fwd(
    dfa: &DFA,
    cache: &mut Cache,
    input: &Input<'_>,
) -> Result<LazyStateID, MatchError> {
    let sid = dfa.start_state_forward(cache, input)?;
    // Start states can never be match states, since all matches are delayed
    // by 1 byte.
    debug_assert!(!sid.is_match());
    Ok(sid)
}