// Answer 0

#[test]
fn test_compile_transition_conflict_with_existing_transition() {
    let config = Config::default();
    let nfa = NFA::default();
    let mut builder = InternalBuilder::new(config, &nfa);
    let dfa_id = StateID::default(); // Valid StateID but already mapped
    let trans = thompson::Transition {
        start: 1,
        end: 5,
        next: StateID::default(), // Not found in NFA
    };
    let epsilons = Epsilons(1); // Example non-zero value
    builder.nfa_to_dfa_id.push(dfa_id); // Simulate that it has been mapped 

    // Simulate an existing transition that would cause a conflict
    let existing_transition = Transition::new(false, StateID::default(), epsilons);
    builder.dfa.set_transition(dfa_id, 1, existing_transition.clone());

    let result = builder.compile_transition(dfa_id, &trans, epsilons);
    // Here we expect a Result::Err(BuildError)
}

#[test]
fn test_compile_transition_with_non_zero_epsilons() {
    let config = Config::default();
    let nfa = NFA::default();
    let mut builder = InternalBuilder::new(config, &nfa);
    let dfa_id = StateID::default(); // Valid StateID but already mapped
    let trans = thompson::Transition {
        start: 2,
        end: 3,
        next: StateID::default(), // Not found in NFA
    };
    let epsilons = Epsilons(2); // Different non-zero values

    builder.nfa_to_dfa_id.push(dfa_id); // Simulate that it has been mapped

    let existing_transition = Transition::new(false, StateID::default(), epsilons);
    builder.dfa.set_transition(dfa_id, 2, existing_transition.clone());

    let result = builder.compile_transition(dfa_id, &trans, epsilons);
    // Here we expect a Result::Err(BuildError)
}

