// Answer 0

#[test]
fn test_truncate_zero() {
    let mut transition_table = TransitionTable {
        table: vec![1, 2, 3, 4],
        classes: ByteClasses([0; 256]),
        stride2: 2,
    };
    transition_table.truncate(0);
}

#[test]
fn test_truncate_one() {
    let mut transition_table = TransitionTable {
        table: vec![1, 2, 3, 4, 5, 6],
        classes: ByteClasses([0; 256]),
        stride2: 1,
    };
    transition_table.truncate(1);
}

#[test]
fn test_truncate_maximum() {
    let mut transition_table = TransitionTable {
        table: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
        classes: ByteClasses([0; 256]),
        stride2: 9,
    };
    transition_table.truncate(512);
}

#[test]
fn test_truncate_mid_range() {
    let mut transition_table = TransitionTable {
        table: vec![1, 2, 3, 4, 5],
        classes: ByteClasses([0; 256]),
        stride2: 4,
    };
    transition_table.truncate(8);
}

#[test]
fn test_truncate_large_input() {
    let mut transition_table = TransitionTable {
        table: vec![0; 1024],
        classes: ByteClasses([0; 256]),
        stride2: 5,
    };
    transition_table.truncate(32);
}

