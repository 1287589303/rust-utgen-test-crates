// Answer 0

#[test]
fn test_write_to_with_non_matching_state() {
    let input_ranges: &[u8] = &[0, 1, 2];
    let next_states: &[u8] = &[0, 1, 2, 3];
    let pattern_ids: &[u8] = &[];
    let accel: &[u8] = &[];

    let state = State {
        id: StateID::default(),
        is_match: false,
        ntrans: 2,
        input_ranges,
        next: next_states,
        pattern_ids,
        accel,
    };

    let mut dst = vec![0; state.write_to_len()];
    let _ = state.write_to::<crate::util::Endian::Little>(&mut dst);
}

#[test]
fn test_write_to_with_matching_state_and_patterns() {
    let input_ranges: &[u8] = &[0, 1, 2];
    let next_states: &[u8] = &[4, 5, 6, 7];
    let pattern_ids: &[u8] = &[1, 2, 3, 4]; // 1 * 4 bytes
    let accel: &[u8] = &[8, 9, 10];

    let state = State {
        id: StateID::default(),
        is_match: true,
        ntrans: 2,
        input_ranges,
        next: next_states,
        pattern_ids,
        accel,
    };

    let mut dst = vec![0; state.write_to_len()];
    let _ = state.write_to::<crate::util::Endian::Little>(&mut dst);
}

