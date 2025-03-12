fn prefilter_restart(
    dfa: &DFA,
    cache: &mut Cache,
    input: &Input<'_>,
    at: usize,
) -> Result<LazyStateID, MatchError> {
    let mut input = input.clone();
    input.set_start(at);
    init_fwd(dfa, cache, &input)
}