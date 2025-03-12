// Answer 0

#[test]
fn test_to_state_id_zero() {
    let table = TransitionTable {
        table: vec![0; 10],
        classes: ByteClasses([0; 256]),
        stride2: 1,
    };
    let state_id = table.to_state_id(0);
}

#[test]
fn test_to_state_id_edge_case_length_minus_one() {
    let len = 10;
    let table = TransitionTable {
        table: vec![0; len],
        classes: ByteClasses([0; 256]),
        stride2: 1,
    };
    let state_id = table.to_state_id(len - 1);
}

#[test]
#[should_panic]
fn test_to_state_id_out_of_bounds_greater_than_len() {
    let len = 10;
    let table = TransitionTable {
        table: vec![0; len],
        classes: ByteClasses([0; 256]),
        stride2: 1,
    };
    let _state_id = table.to_state_id(len);
}

#[test]
#[should_panic]
fn test_to_state_id_out_of_bounds_negative() {
    let len = 10;
    let table = TransitionTable {
        table: vec![0; len],
        classes: ByteClasses([0; 256]),
        stride2: 1,
    };
    let _state_id = table.to_state_id((usize::MAX) - 1);
}

#[test]
fn test_to_state_id_capacity_limit() {
    let table = TransitionTable {
        table: vec![0; 1 << 30], // Large size to reach capacity limits of usize
        classes: ByteClasses([0; 256]),
        stride2: 1,
    };
    let state_id = table.to_state_id(0);
    let state_id_edge = table.to_state_id((1 << 30) - 1);
}

