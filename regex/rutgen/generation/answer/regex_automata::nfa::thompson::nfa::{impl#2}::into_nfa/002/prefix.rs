// Answer 0

#[test]
fn test_into_nfa_with_single_match() {
    let mut inner = Inner::default();
    let start_state_id = StateID::default();
    let match_state_id = StateID::default();
    
    // Initializing the states with a match state.
    inner.states.push(State::Match { pattern_id: PatternID::default() });
    inner.start_pattern.push(start_state_id);
    inner.start_anchored = start_state_id;
    inner.start_unanchored = start_state_id;

    let mut stack = vec![];
    let mut seen = SparseSet::new(1);
    
    // Start with the match state.
    stack.push(match_state_id);

    while let Some(sid) = stack.pop() {
        if !seen.insert(sid) {
            continue;
        }

        match inner.states[sid] {
            State::Match { .. } => inner.has_empty = true,
            _ => continue,
        }
    }
    
    let nfa = inner.into_nfa();
}

#[test]
fn test_into_nfa_exceeding_start_pattern() {
    let mut inner = Inner::default();
    let start_state_id = StateID::default();
    let match_state_id = StateID::default();

    // Setting up states and match state
    inner.states.push(State::Match { pattern_id: PatternID::default() });
    inner.start_pattern.push(start_state_id);
    inner.start_anchored = start_state_id;
    inner.start_unanchored = start_state_id;

    let mut stack = vec![];
    let mut seen = SparseSet::new(1);

    // Initialize the stack with a state.
    stack.push(match_state_id);

    // Simulate popping from stack for an unmatched start_id
    while let Some(sid) = stack.pop() {
        if !seen.insert(sid) {
            continue;
        }

        match inner.states[sid] {
            State::Match { .. } => inner.has_empty = true,
            _ => continue,
        }
    }
    
    // Pushing to stack again to exceed start_pattern
    stack.push(StateID::default());
    
    let nfa = inner.into_nfa();
}

#[test]
fn test_into_nfa_with_multiple_states() {
    let mut inner = Inner::default();
    let start_state_id = StateID::default();
    let match_state_id_1 = StateID::new_unchecked(1);
    let match_state_id_2 = StateID::new_unchecked(2);
    
    // Setting up states with multiple match states.
    inner.states.push(State::Match { pattern_id: PatternID::default() });
    inner.states.push(State::Match { pattern_id: PatternID::default() });
    inner.start_pattern.push(start_state_id);
    inner.start_pattern.push(StateID::default());
    inner.start_anchored = start_state_id;
    inner.start_unanchored = start_state_id;

    let mut stack = vec![];
    let mut seen = SparseSet::new(2);

    // Start with the first match state.
    stack.push(match_state_id_1);
    stack.push(match_state_id_2);

    while let Some(sid) = stack.pop() {
        if !seen.insert(sid) {
            continue;
        }

        match inner.states[sid] {
            State::Match { .. } => inner.has_empty = true,
            _ => continue,
        }
    }
    
    let nfa = inner.into_nfa();
}

