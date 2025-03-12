// Answer 0

#[test]
fn test_sparse_mut_empty() {
    let mut transitions = Transitions {
        sparse: vec![].into_boxed_slice(), // 0 bytes
        classes: ByteClasses([0; 256]),
        state_len: 0,
        pattern_len: 0,
    };
    let _result = transitions.sparse_mut();
}

#[test]
fn test_sparse_mut_single_byte() {
    let mut transitions = Transitions {
        sparse: vec![0_u8].into_boxed_slice(), // 1 byte
        classes: ByteClasses([0; 256]),
        state_len: 1,
        pattern_len: 0,
    };
    let _result = transitions.sparse_mut();
}

#[test]
fn test_sparse_mut_max_bytes() {
    let mut transitions = Transitions {
        sparse: vec![0_u8; 256].into_boxed_slice(), // 256 bytes
        classes: ByteClasses([0; 256]),
        state_len: 256,
        pattern_len: 256,
    };
    let _result = transitions.sparse_mut();
}

#[test]
fn test_sparse_mut_multiple_bytes() {
    let mut transitions = Transitions {
        sparse: vec![1_u8; 128].into_boxed_slice(), // 128 bytes
        classes: ByteClasses([0; 256]),
        state_len: 128,
        pattern_len: 64,
    };
    let _result = transitions.sparse_mut();
}

