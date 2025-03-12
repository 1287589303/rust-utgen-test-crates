fn prefilter_restart<A: Automaton + ?Sized>(
    dfa: &A,
    input: &Input<'_>,
    at: usize,
) -> Result<StateID, MatchError> {
    let mut input = input.clone();
    input.set_start(at);
    init_fwd(dfa, &input)
}