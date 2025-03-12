// Answer 0

#[test]
fn test_to_index_valid_case_1() {
    let stride2 = 1;
    let state_id = StateID(0); // 0 as usize
    let transition_table = TransitionTable {
        table: vec![0; 8], // dummy data
        classes: ByteClasses([0; 256]),
        stride2,
    };
    let _ = transition_table.to_index(state_id);
}

#[test]
fn test_to_index_valid_case_2() {
    let stride2 = 5;
    let state_id = StateID(32); // 32 as usize
    let transition_table = TransitionTable {
        table: vec![0; 8], // dummy data
        classes: ByteClasses([0; 256]),
        stride2,
    };
    let _ = transition_table.to_index(state_id);
}

#[test]
fn test_to_index_valid_case_3() {
    let stride2 = 9;
    let state_id = StateID(256); // 256 as usize
    let transition_table = TransitionTable {
        table: vec![0; 8], // dummy data
        classes: ByteClasses([0; 256]),
        stride2,
    };
    let _ = transition_table.to_index(state_id);
}

#[test]
#[should_panic]
fn test_to_index_invalid_case() {
    let stride2 = 9;
    let state_id = StateID(512); // 512 is out of valid range for as_usize
    let transition_table = TransitionTable {
        table: vec![0; 8], // dummy data
        classes: ByteClasses([0; 256]),
        stride2,
    };
    let _ = transition_table.to_index(state_id);
}

