// Answer 0

#[test]
fn test_epsilon_closure_look_failed_condition() {
    let mut stack: Vec<StateID> = Vec::new();
    let look_have = LookSet::full(); // Assuming full look set for the test
    let start_nfa_id = StateID(0); // Assuming a valid StateID
    
    // Construct a mock NFA with an initial epsilon state
    let mut nfa = thompson::NFA::new("some_pattern").unwrap();
    // Create a look state that has some look transitions
    nfa.states.push(State::Look {
        look: Look::Start,
        next: StateID(1), // The next state must also exist in nfa
    });
    nfa.states.push(State::Look {
        look: Look::End,
        next: StateID(2), // The next state
    });
    nfa.states.push(State::Match {
        pattern_id: PatternID(0),
    });
    
    let mut set = SparseSet::new(10); // Capacity for the sparse set

    // Now we can call the epsilon_closure function
    epsilon_closure(&nfa, start_nfa_id, look_have, &mut stack, &mut set);
}

#[test]
fn test_epsilon_closure_multiple_states() {
    let mut stack: Vec<StateID> = Vec::new();
    let look_have = LookSet::empty(); // Should not contain any look
    let start_nfa_id = StateID(0); // Start state

    // Construct a mock NFA with epsilon transitions
    let mut nfa = thompson::NFA::new("some_pattern").unwrap();
    nfa.states.push(State::Look {
        look: Look::WordStartAscii,
        next: StateID(1), // Push to stack
    });
    nfa.states.push(State::Look {
        look: Look::WordEndAscii,
        next: StateID(2), // Push to stack but won't be followed
    });
    nfa.states.push(State::Match {
        pattern_id: PatternID(0),
    });

    let mut set = SparseSet::new(10); // Capacity for the sparse set

    // Call the epsilon_closure function
    epsilon_closure(&nfa, start_nfa_id, look_have, &mut stack, &mut set);
}

