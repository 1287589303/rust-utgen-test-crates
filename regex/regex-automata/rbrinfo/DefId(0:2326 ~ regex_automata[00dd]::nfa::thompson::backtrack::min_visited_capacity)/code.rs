pub fn min_visited_capacity(nfa: &NFA, input: &Input<'_>) -> usize {
    div_ceil(nfa.states().len() * (input.get_span().len() + 1), 8)
}