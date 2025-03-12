// Answer 0

#[test]
fn test_is_valid_zero_stride() {
    let transition_table = TransitionTable {
        table: vec![0; 512],
        classes: ByteClasses([0; 256]),
        stride2: 1,
    };
    let id = StateID(0);
    transition_table.is_valid(id);
}

#[test]
fn test_is_valid_at_boundary_stride2() {
    let transition_table = TransitionTable {
        table: vec![0; 256],
        classes: ByteClasses([0; 256]),
        stride2: 8,
    };
    let id = StateID(0);
    transition_table.is_valid(id);
}

#[test]
fn test_is_valid_even_ids_stride2() {
    let transition_table = TransitionTable {
        table: vec![0; 512],
        classes: ByteClasses([0; 256]),
        stride2: 3,
    };
    let id = StateID(2);
    transition_table.is_valid(id);
}

#[test]
fn test_is_valid_odd_ids_stride2() {
    let transition_table = TransitionTable {
        table: vec![0; 512],
        classes: ByteClasses([0; 256]),
        stride2: 4,
    };
    let id = StateID(4);
    transition_table.is_valid(id);
}

#[test]
fn test_is_valid_exceeding_length() {
    let transition_table = TransitionTable {
        table: vec![0; 256],
        classes: ByteClasses([0; 256]),
        stride2: 7,
    };
    let id = StateID(512);
    transition_table.is_valid(id);
}

