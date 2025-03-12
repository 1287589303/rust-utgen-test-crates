// Answer 0

#[test]
fn test_write_to_with_bound_length_and_no_transitions() {
    let input_ranges: &[u8] = &[];
    let next: &[u8] = &[];
    let pattern_ids: &[u8] = &[];
    let accel: &[u8] = &[];

    let state = State {
        id: StateID::default(),
        is_match: true,
        ntrans: 0,
        input_ranges,
        next,
        pattern_ids,
        accel,
    };

    let nwrite = state.write_to_len();
    let mut dst = vec![0; nwrite];

    let result = state.write_to::<wire::LittleEndian>(&mut dst);
}

#[test]
fn test_write_to_with_non_matching_state() {
    let input_ranges: &[u8] = &[];
    let next: &[u8] = &[];
    let pattern_ids: &[u8] = &[];
    let accel: &[u8] = &[];

    let state = State {
        id: StateID::default(),
        is_match: false,
        ntrans: 0,
        input_ranges,
        next,
        pattern_ids,
        accel,
    };

    let nwrite = state.write_to_len();
    let mut dst = vec![0; nwrite];

    let result = state.write_to::<wire::LittleEndian>(&mut dst);
}

