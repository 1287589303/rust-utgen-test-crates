// Answer 0

#[test]
fn test_stride_min_value() {
    let stride2 = 1;
    let transition_table = TransitionTable {
        table: vec![0; 1 << stride2],
        classes: ByteClasses([0; 256]),
        stride2,
    };
    let _ = transition_table.stride();
}

#[test]
fn test_stride_mid_value() {
    let stride2 = 5;
    let transition_table = TransitionTable {
        table: vec![0; 1 << stride2],
        classes: ByteClasses([0; 256]),
        stride2,
    };
    let _ = transition_table.stride();
}

#[test]
fn test_stride_max_value() {
    let stride2 = 9;
    let transition_table = TransitionTable {
        table: vec![0; 1 << stride2],
        classes: ByteClasses([0; 256]),
        stride2,
    };
    let _ = transition_table.stride();
}

