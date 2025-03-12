// Answer 0

#[test]
fn test_swap_valid_states_no_transitions() {
    let byte_classes = ByteClasses::singletons();
    let mut transition_table = TransitionTable {
        table: vec![0; 0], // No transitions
        classes: byte_classes,
        stride2: 1,
    };
    
    let id1 = StateID(0.into());
    let id2 = StateID(1.into());
    
    transition_table.add_empty_state().unwrap(); // Add first state
    transition_table.add_empty_state().unwrap(); // Add second state

    transition_table.swap(id1, id2);
}

#[test]
#[should_panic]
fn test_swap_invalid_state_id1() {
    let byte_classes = ByteClasses::singletons();
    let mut transition_table = TransitionTable {
        table: vec![0; 0], // No transitions
        classes: byte_classes,
        stride2: 1,
    };
    
    transition_table.add_empty_state().unwrap(); // Add first state

    let id1 = StateID(2.into()); // Invalid
    let id2 = StateID(0.into()); 

    transition_table.swap(id1, id2);
}

#[test]
#[should_panic]
fn test_swap_invalid_state_id2() {
    let byte_classes = ByteClasses::singletons();
    let mut transition_table = TransitionTable {
        table: vec![0; 0], // No transitions
        classes: byte_classes,
        stride2: 1,
    };
    
    transition_table.add_empty_state().unwrap(); // Add first state

    let id1 = StateID(0.into()); 
    let id2 = StateID(2.into()); // Invalid

    transition_table.swap(id1, id2);
}

