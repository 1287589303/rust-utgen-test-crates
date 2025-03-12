// Answer 0

#[test]
fn test_swap_valid_invalid_state_ids() {
    let mut transition_table = TransitionTable {
        table: vec![0u32; 10], // Creating a transition table with 10 dummy entries
        classes: ByteClasses([0; 256]), // Initializing ByteClasses with zeros
        stride2: 1, // Setting stride2 to minimum value
    };
    
    let id1 = StateID(0); // Valid state ID
    // id2 is intentionally set to an invalid state ID (out of bounds)
    let id2 = StateID(10); // Assuming out of bounds of created table

    transition_table.swap(id1, id2);
}

#[test]
#[should_panic]
fn test_swap_invalid_id2() {
    let mut transition_table = TransitionTable {
        table: vec![0u32; 10],
        classes: ByteClasses([0; 256]),
        stride2: 1,
    };
    
    let id1 = StateID(0); // Valid state ID
    let id2 = StateID(10); // Invalid state ID intentionally

    transition_table.swap(id1, id2);
}

