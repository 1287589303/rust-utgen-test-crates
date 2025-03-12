// Answer 0

#[test]
fn test_next_with_valid_id() {
    struct TestTransition {
        sparse: Vec<u8>,
        classes: ByteClasses,
        state_len: usize,
        pattern_len: usize,
    }

    let transitions = TestTransition {
        sparse: vec![0u8; 10], // Ensure non-empty sparse (10 bytes)
        classes: ByteClasses::new(),
        state_len: 1,
        pattern_len: 0,
    };

    let id = StateID(0); // Valid ID within range
    let state_iter = StateIter {
        trans: &transitions,
        id: id.0,
    };

    let _ = state_iter.next();
}

#[test]
fn test_next_with_max_id() {
    struct TestTransition {
        sparse: Vec<u8>,
        classes: ByteClasses,
        state_len: usize,
        pattern_len: usize,
    }

    let sparse_data = vec![0u8; 10]; // Ensure non-empty sparse (10 bytes)
    let transitions = TestTransition {
        sparse: sparse_data.clone(),
        classes: ByteClasses::new(),
        state_len: 1,
        pattern_len: 0,
    };

    let id = StateID((transitions.sparse.len() / 2) as usize - 1); // Max valid ID
    let state_iter = StateIter {
        trans: &transitions,
        id: id.0,
    };

    let _ = state_iter.next();
}

