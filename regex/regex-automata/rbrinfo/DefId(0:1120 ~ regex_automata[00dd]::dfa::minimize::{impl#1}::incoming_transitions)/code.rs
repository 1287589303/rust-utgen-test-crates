fn incoming_transitions(dfa: &dense::OwnedDFA) -> Vec<Vec<Vec<StateID>>> {
        let mut incoming = vec![];
        for _ in dfa.states() {
            incoming.push(vec![vec![]; dfa.alphabet_len()]);
        }
        for state in dfa.states() {
            for (b, next) in state.transitions() {
                incoming[dfa.to_index(next)][b.as_usize()].push(state.id());
            }
        }
        incoming
    }