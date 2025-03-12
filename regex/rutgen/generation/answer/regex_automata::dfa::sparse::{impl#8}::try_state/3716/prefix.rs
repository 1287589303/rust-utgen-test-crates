// Answer 0

#[test]
fn test_try_state_invalid_transition_length() {
    #[derive(Clone)]
    struct TestTransitions {
        sparse: Vec<u8>,
        classes: ByteClasses,
        state_len: usize,
        pattern_len: usize,
    }

    let id = StateID(0);
    let sparse_data = vec![0u8; 2 + 2 + 2 * 257 + StateID::SIZE]; // Sparse data
    let ntrans = 0; // for the edge case
    let mut sparse = sparse_data.clone();
    sparse[0..2].copy_from_slice(&(ntrans | (0 << 15) as u16).to_le_bytes());
    
    let transitions = TestTransitions {
        sparse: sparse.clone(),
        classes: ByteClasses([0; 256]),
        state_len: 1,
        pattern_len: 0,
    };

    let special = Special {
        max: id,
        quit_id: id,
        min_match: id,
        max_match: id,
        min_accel: id,
        max_accel: id,
        min_start: id,
        max_start: id,
    };

    let _ = transitions.try_state(&special, id);
}

