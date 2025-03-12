// Answer 0

#[test]
fn test_add_nfa_states_with_look_state_empty_look_need() {
    let nfa = thompson::NFA::always_match();
    let mut builder = StateBuilderNFA { repr: Vec::new(), prev_nfa_state_id: StateID(0) };
    let mut set = SparseSet::new(1);
    let look_state_id = StateID(0);
    
    builder.set_look_need(|need| need.insert(Look::Start));
    set.insert(look_state_id);
    
    add_nfa_states(&nfa, &set, &mut builder);
}

#[test]
fn test_add_nfa_states_with_look_state_non_empty_look_need() {
    let nfa = thompson::NFA::always_match();
    let mut builder = StateBuilderNFA { repr: Vec::new(), prev_nfa_state_id: StateID(0) };
    let mut set = SparseSet::new(1);
    let look_state_id = StateID(0);
    
    builder.set_look_need(|need| need.insert(Look::End));
    set.insert(look_state_id);
    
    add_nfa_states(&nfa, &set, &mut builder);
}

#[test]
fn test_add_nfa_states_with_non_look_state() {
    let nfa = thompson::NFA::always_match();
    let mut builder = StateBuilderNFA { repr: Vec::new(), prev_nfa_state_id: StateID(0) };
    let mut set = SparseSet::new(1);
    let non_look_state_id = StateID(1);
    
    set.insert(non_look_state_id);
    
    add_nfa_states(&nfa, &set, &mut builder);
}

