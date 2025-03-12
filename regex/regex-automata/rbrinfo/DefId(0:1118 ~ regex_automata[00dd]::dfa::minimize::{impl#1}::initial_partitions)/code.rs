fn initial_partitions(dfa: &dense::OwnedDFA) -> Vec<StateSet> {
        // For match states, we know that two match states with different
        // pattern ID lists will *always* be distinct, so we can partition them
        // initially based on that.
        let mut matching: BTreeMap<Vec<PatternID>, StateSet> = BTreeMap::new();
        let mut is_quit = StateSet::empty();
        let mut no_match = StateSet::empty();
        for state in dfa.states() {
            if dfa.is_match_state(state.id()) {
                let mut pids = vec![];
                for i in 0..dfa.match_len(state.id()) {
                    pids.push(dfa.match_pattern(state.id(), i));
                }
                matching
                    .entry(pids)
                    .or_insert(StateSet::empty())
                    .add(state.id());
            } else if dfa.is_quit_state(state.id()) {
                is_quit.add(state.id());
            } else {
                no_match.add(state.id());
            }
        }

        let mut sets: Vec<StateSet> =
            matching.into_iter().map(|(_, set)| set).collect();
        sets.push(no_match);
        sets.push(is_quit);
        sets
    }