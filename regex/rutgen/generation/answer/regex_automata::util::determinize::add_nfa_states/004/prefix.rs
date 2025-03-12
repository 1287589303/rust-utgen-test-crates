// Answer 0

#[test]
fn test_add_nfa_states_with_fail_state() {
    let nfa_id_fail = StateID(1);
    let nfa_id_byte_range = StateID(2);
    let nfa_id_dense = StateID(3);

    let mut builder = StateBuilderNFA {
        repr: vec![],
        prev_nfa_state_id: StateID(0),
    };
    
    builder.set_look_need(|_| LookSet::singleton(Look::Start));
    
    let mut sparse_set = SparseSet::new(3);
    sparse_set.insert(nfa_id_fail);
    sparse_set.insert(nfa_id_byte_range);
    sparse_set.insert(nfa_id_dense);

    let nfa = thompson::NFA::always_match();
    
    add_nfa_states(&nfa, &sparse_set, &mut builder);
}

#[test]
fn test_add_nfa_states_with_multiple_fail_states() {
    let nfa_id_fail1 = StateID(1);
    let nfa_id_fail2 = StateID(2);
    let nfa_id_sparse = StateID(3);

    let mut builder = StateBuilderNFA {
        repr: vec![],
        prev_nfa_state_id: StateID(0),
    };
    
    builder.set_look_need(|_| LookSet::singleton(Look::End));

    let mut sparse_set = SparseSet::new(3);
    sparse_set.insert(nfa_id_fail1);
    sparse_set.insert(nfa_id_fail2);
    sparse_set.insert(nfa_id_sparse);

    let nfa = thompson::NFA::never_match();
    
    add_nfa_states(&nfa, &sparse_set, &mut builder);
}

#[test]
fn test_add_nfa_states_mixed_state_types() {
    let nfa_id_fail = StateID(1);
    let nfa_id_look = StateID(2);
    let nfa_id_dense = StateID(3);

    let mut builder = StateBuilderNFA {
        repr: vec![],
        prev_nfa_state_id: StateID(0),
    };
    
    builder.set_look_need(|_| LookSet::singleton(Look::WordAscii));

    let mut sparse_set = SparseSet::new(3);
    sparse_set.insert(nfa_id_fail);
    sparse_set.insert(nfa_id_look);
    sparse_set.insert(nfa_id_dense);

    let nfa = thompson::NFA::always_match();
    
    add_nfa_states(&nfa, &sparse_set, &mut builder);
}

