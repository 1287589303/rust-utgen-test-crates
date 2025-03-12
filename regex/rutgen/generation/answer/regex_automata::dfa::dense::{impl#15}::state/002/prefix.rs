// Answer 0

#[test]
#[should_panic]
fn test_state_invalid_id_negative() {
    let id = StateID(-1.try_into().unwrap()); // Negative value, invalid state ID
    let table: Vec<u32> = vec![0; 10]; // Example table with 10 entries
    let classes = ByteClasses([0; 256]);
    let transition_table = TransitionTable {
        table,
        classes,
        stride2: 1,
    };
    transition_table.state(id);
}

#[test]
#[should_panic]
fn test_state_invalid_id_too_large() {
    let id = StateID(10.try_into().unwrap()); // ID greater than the table length
    let table: Vec<u32> = vec![0; 10]; // Example table with 10 entries
    let classes = ByteClasses([0; 256]);
    let transition_table = TransitionTable {
        table,
        classes,
        stride2: 1,
    };
    transition_table.state(id);
}

#[test]
#[should_panic]
fn test_state_invalid_id_not_aligned() {
    let id = StateID(1.try_into().unwrap()); // ID not aligned with stride
    let table: Vec<u32> = vec![0; 10]; // Example table with 10 entries
    let classes = ByteClasses([0; 256]);
    let transition_table = TransitionTable {
        table,
        classes,
        stride2: 2, // Stride set to 2, which expects even alignment
    };
    transition_table.state(id);
}

