fn init_fwd<A: Automaton + ?Sized>(
    dfa: &A,
    input: &Input<'_>,
) -> Result<StateID, MatchError> {
    let sid = dfa.start_state_forward(input)?;
    // Start states can never be match states, since all matches are delayed
    // by 1 byte.
    debug_assert!(!dfa.is_match_state(sid));
    Ok(sid)
}