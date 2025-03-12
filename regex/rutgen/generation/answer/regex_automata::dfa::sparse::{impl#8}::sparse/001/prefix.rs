// Answer 0

#[test]
fn test_sparse_single_byte() {
    let sparse_data: [u8; 1] = [42];
    let transitions = Transitions {
        sparse: &sparse_data,
        classes: ByteClasses([0; 256]),
        state_len: 1,
        pattern_len: 1,
    };
    let _ = transitions.sparse();
}

#[test]
fn test_sparse_multiple_bytes() {
    let sparse_data: [u8; 5] = [10, 20, 30, 40, 50];
    let transitions = Transitions {
        sparse: &sparse_data,
        classes: ByteClasses([0; 256]),
        state_len: 5,
        pattern_len: 5,
    };
    let _ = transitions.sparse();
}

#[test]
fn test_sparse_maximum_size() {
    let sparse_data: [u8; 256] = [0; 256]; // All zeros
    let transitions = Transitions {
        sparse: &sparse_data,
        classes: ByteClasses([0; 256]),
        state_len: 256,
        pattern_len: 256,
    };
    let _ = transitions.sparse();
}

