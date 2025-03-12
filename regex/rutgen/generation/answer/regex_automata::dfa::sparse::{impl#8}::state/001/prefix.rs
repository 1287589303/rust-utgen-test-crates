// Answer 0

#[test]
fn test_state_with_valid_state_id_and_match() {
    struct TestTransitions {
        sparse: Vec<u8>,
        classes: ByteClasses,
        state_len: usize,
        pattern_len: usize,
    }

    let id = StateID(1);
    let ntrans = 2; // ntrans > 0
    let input_ranges = vec![0, 1, 2, 3]; // length = ntrans * 2
    let next = vec![0, 1, 0, 1]; // length = ntrans * StateID::SIZE
    let pattern_ids = vec![0, 0, 1, 1]; // length = npats * 4
    let accel = vec![0, 1, 2]; // length <= 3
    let is_match = true;

    let mut sparse = vec![0; 6 + pattern_ids.len()]; // Enough space for state encoding
    sparse[0..2].copy_from_slice(&0u16.to_ne_bytes()); // ntrans placeholder
    sparse[2] = (is_match as u16 | (ntrans as u16) << 15) as u8; // set ntrans and is_match
    sparse[3..5].copy_from_slice(&next); // Transition states
    sparse[5..5 + input_ranges.len()].copy_from_slice(&input_ranges); // Input ranges
    sparse[5 + input_ranges.len()..5 + input_ranges.len() + pattern_ids.len()].copy_from_slice(&pattern_ids); // Pattern IDs
    sparse[5 + input_ranges.len() + pattern_ids.len()] = accel.len() as u8; // accel length
    sparse[6 + input_ranges.len() + pattern_ids.len()..].copy_from_slice(&accel); // Accel

    let transitions = TestTransitions {
        sparse,
        classes: ByteClasses([0; 256]),
        state_len: 2,
        pattern_len: 2,
    };

    let state = transitions.state(id);
}

#[test]
#[should_panic]
fn test_state_with_invalid_id() {
    struct TestTransitions {
        sparse: Vec<u8>,
        classes: ByteClasses,
        state_len: usize,
        pattern_len: usize,
    }

    let id = StateID(2); // Invalid ID as it's out of range
    let ntrans = 1;
    let input_ranges = vec![0, 1];
    let next = vec![0];
    let pattern_ids = vec![0, 1, 2, 3];
    let accel = vec![0];

    let mut sparse = vec![0; 6 + pattern_ids.len()];
    sparse[0..2].copy_from_slice(&1u16.to_ne_bytes());
    sparse[2] = (1 << 15) as u8; // is_match = true
    sparse[3..5].copy_from_slice(&next);
    sparse[5..5 + input_ranges.len()].copy_from_slice(&input_ranges);
    sparse[5 + input_ranges.len()..5 + input_ranges.len() + pattern_ids.len()].copy_from_slice(&pattern_ids);
    sparse[5 + input_ranges.len() + pattern_ids.len()] = accel.len() as u8;
    sparse[6 + input_ranges.len() + pattern_ids.len()..].copy_from_slice(&accel);

    let transitions = TestTransitions {
        sparse,
        classes: ByteClasses([0; 256]),
        state_len: 2,
        pattern_len: 2,
    };

    let state = transitions.state(id); // This will panic
}

