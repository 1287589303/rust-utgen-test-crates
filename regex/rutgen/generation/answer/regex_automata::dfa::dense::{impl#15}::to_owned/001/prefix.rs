// Answer 0

#[test]
fn test_to_owned_non_empty_table() {
    let table: &[u32] = &[1, 2, 3, 4, 5]; // length 5
    let classes = ByteClasses([0; 256]); // valid initialization
    let stride2: usize = 3; // within range [1, 9]

    let transition_table = TransitionTable {
        table: table,
        classes: classes,
        stride2: stride2,
    };

    let owned_transition_table = transition_table.to_owned();
}

#[test]
fn test_to_owned_minimum_table_length() {
    let table: &[u32] = &[1]; // length 1
    let classes = ByteClasses([0; 256]); // valid initialization
    let stride2: usize = 1; // minimum value

    let transition_table = TransitionTable {
        table: table,
        classes: classes,
        stride2: stride2,
    };

    let owned_transition_table = transition_table.to_owned();
}

#[test]
fn test_to_owned_maximum_table_length() {
    let table: &[u32] = &[0; 257]; // length 257
    let classes = ByteClasses([0; 256]); // valid initialization
    let stride2: usize = 9; // maximum value

    let transition_table = TransitionTable {
        table: table,
        classes: classes,
        stride2: stride2,
    };

    let owned_transition_table = transition_table.to_owned();
}

#[test]
fn test_to_owned_stride2_bounds() {
    let table: &[u32] = &[1, 2, 3]; // length 3
    let classes = ByteClasses([0; 256]); // valid initialization
    let stride2: usize = 5; // within range [1, 9]

    let transition_table = TransitionTable {
        table: table,
        classes: classes,
        stride2: stride2,
    };

    let owned_transition_table = transition_table.to_owned();
}

