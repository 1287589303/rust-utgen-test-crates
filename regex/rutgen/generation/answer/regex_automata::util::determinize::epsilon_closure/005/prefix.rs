// Answer 0

#[test]
fn test_epsilon_closure_binary_union_insert_true() {
    let mut stack = Vec::new();
    let mut set = SparseSet::new(10);
    
    let mut nfa = thompson::NFA::new("a|b").unwrap();
    let start_nfa_id = nfa.start_unanchored();
    
    stack.push(start_nfa_id);
    let look_have = LookSet::empty();

    epsilon_closure(&nfa, start_nfa_id, look_have, &mut stack, &mut set);
}

#[test]
fn test_epsilon_closure_binary_union_insert_false() {
    let mut stack = Vec::new();
    let mut set = SparseSet::new(10);
    
    let mut nfa = thompson::NFA::new("a|b").unwrap();
    let start_nfa_id = nfa.start_unanchored();
    
    stack.push(start_nfa_id);
    let look_have = LookSet::empty();

    // Perform first call to populate set
    epsilon_closure(&nfa, start_nfa_id, look_have, &mut stack, &mut set);

    // Call again to check for already inserted states
    epsilon_closure(&nfa, start_nfa_id, look_have, &mut stack, &mut set);
}

