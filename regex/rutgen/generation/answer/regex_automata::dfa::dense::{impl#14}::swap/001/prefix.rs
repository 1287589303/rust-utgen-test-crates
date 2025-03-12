// Answer 0

#[test]
fn test_swap_valid_states() {
    let mut classes = ByteClasses([0; 256]);
    classes.0[255] = 1; // Setting alphabet length to 1
    let mut table = vec![0; 1]; // Transition table for alphabet length of 1
    let mut transition_table = TransitionTable { table, classes, stride2: 1 };
    
    let id1 = StateID(SmallIndex::new(0));
    let id2 = StateID(SmallIndex::new(1));
    
    transition_table.swap(id1, id2);
}

#[test]
#[should_panic]
fn test_swap_invalid_state_id1() {
    let mut classes = ByteClasses([0; 256]);
    classes.0[255] = 1; // Setting alphabet length to 1
    let mut table = vec![0; 1]; // Transition table for alphabet length of 1
    let mut transition_table = TransitionTable { table, classes, stride2: 1 };
    
    let id1 = StateID(SmallIndex::new(255)); // Out of bounds
    let id2 = StateID(SmallIndex::new(1));
    
    transition_table.swap(id1, id2);
}

#[test]
#[should_panic]
fn test_swap_invalid_state_id2() {
    let mut classes = ByteClasses([0; 256]);
    classes.0[255] = 1; // Setting alphabet length to 1
    let mut table = vec![0; 1]; // Transition table for alphabet length of 1
    let mut transition_table = TransitionTable { table, classes, stride2: 1 };
    
    let id1 = StateID(SmallIndex::new(0));
    let id2 = StateID(SmallIndex::new(255)); // Out of bounds
    
    transition_table.swap(id1, id2);
}

#[test]
#[should_panic]
fn test_swap_empty_alphabet() {
    let classes = ByteClasses([0; 256]);
    let mut table = vec![]; // Transition table for alphabet length of 0
    let mut transition_table = TransitionTable { table, classes, stride2: 1 };
    
    let id1 = StateID(SmallIndex::new(0));
    let id2 = StateID(SmallIndex::new(1));
    
    transition_table.swap(id1, id2);
}

#[test]
fn test_swap_large_alphabet() {
    let mut classes = ByteClasses([0; 256]);
    classes.0[255] = 256; // Setting alphabet length to 256
    let mut table = vec![0; 512]; // Transition table for alphabet length of 256
    let mut transition_table = TransitionTable { table, classes, stride2: 9 };
    
    let id1 = StateID(SmallIndex::new(0));
    let id2 = StateID(SmallIndex::new(1));
    
    transition_table.swap(id1, id2);
}

