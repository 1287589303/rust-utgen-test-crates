// Answer 0

#[test]
fn test_epsilon_closure_dense_state() {
    use crate::util::{SparseSet, LookSet};
    use crate::nfa::thompson::{self, State, StateID};

    let mut stack = Vec::new();
    let mut set = SparseSet::new(10);
    let look_have = LookSet::empty();
    
    let mut nfa = thompson::NFA::never_match(); // Starts with an empty NFA
    let start_nfa_id = StateID(0);

    // Adding a Dense state
    nfa.states.push(State::Dense(thompson::DenseTransitions::new(vec![
        // Define a dense transition here
    ])));
    
    // Ensure that the start_nfa_id corresponds to a valid epsilon state
    assert!(nfa.state(start_nfa_id).is_epsilon());
    
    // Call the function under test
    epsilon_closure(&nfa, start_nfa_id, look_have, &mut stack, &mut set);
}

#[test]
fn test_epsilon_closure_sparse_state() {
    use crate::util::{SparseSet, LookSet};
    use crate::nfa::thompson::{self, State, StateID};

    let mut stack = Vec::new();
    let mut set = SparseSet::new(10);
    let look_have = LookSet::singleton(Look::Start);

    let mut nfa = thompson::NFA::never_match(); // Starts with an empty NFA
    let start_nfa_id = StateID(1);

    // Adding a Sparse state
    nfa.states.push(State::Sparse(thompson::SparseTransitions::new(vec![
        // Define sparse transitions here
    ])));
    
    // Ensure that the start_nfa_id corresponds to a valid epsilon state
    assert!(nfa.state(start_nfa_id).is_epsilon());
    
    // Call the function under test
    epsilon_closure(&nfa, start_nfa_id, look_have, &mut stack, &mut set);
}

#[test]
fn test_epsilon_closure_byte_range_state() {
    use crate::util::{SparseSet, LookSet};
    use crate::nfa::thompson::{self, State, StateID};

    let mut stack = Vec::new();
    let mut set = SparseSet::new(10);
    let look_have = LookSet::full();

    let mut nfa = thompson::NFA::never_match(); // Starts with an empty NFA
    let start_nfa_id = StateID(2);

    // Adding a ByteRange state
    nfa.states.push(State::ByteRange {
        trans: thompson::Transition::new(0, 1), // define the transition details
    });

    // Ensure that the previously added state is an epsilon state
    assert!(nfa.state(start_nfa_id).is_epsilon());
    
    // Call the function under test
    epsilon_closure(&nfa, start_nfa_id, look_have, &mut stack, &mut set);
}

#[test]
fn test_epsilon_closure_match_state() {
    use crate::util::{SparseSet, LookSet};
    use crate::nfa::thompson::{self, StateID};

    let mut stack = Vec::new();
    let mut set = SparseSet::new(10);
    let look_have = LookSet::empty();

    let mut nfa = thompson::NFA::never_match(); // Starts with an empty NFA
    let start_nfa_id = StateID(3);

    // Adding a Match state
    nfa.states.push(State::Match {
        pattern_id: PatternID(0), // use a defined pattern id
    });

    // Ensure that the match state is epsilon
    assert!(nfa.state(start_nfa_id).is_epsilon());
    
    // Call the function under test
    epsilon_closure(&nfa, start_nfa_id, look_have, &mut stack, &mut set);
}

#[test]
fn test_epsilon_closure_fail_state() {
    use crate::util::{SparseSet, LookSet};
    use crate::nfa::thompson::{self, StateID};

    let mut stack = Vec::new();
    let mut set = SparseSet::new(10);
    let look_have = LookSet::empty();

    let mut nfa = thompson::NFA::never_match(); // Starts with an empty NFA
    let start_nfa_id = StateID(4);

    // Adding a Fail state
    nfa.states.push(State::Fail);

    // Ensure that the fail state is epsilon
    assert!(nfa.state(start_nfa_id).is_epsilon());
    
    // Call the function under test
    epsilon_closure(&nfa, start_nfa_id, look_have, &mut stack, &mut set);
}

