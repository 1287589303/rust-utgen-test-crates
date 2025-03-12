// Answer 0

#[test]
fn test_fmt_with_zero_transitions() {
    let state_id = StateID(1);
    let input_ranges = &[];
    let next = &[0u8];
    let pattern_ids = &[];
    let accel = &[];

    let state = State {
        id: state_id,
        is_match: false,
        ntrans: 1,
        input_ranges,
        next,
        pattern_ids,
        accel,
    };
    let mut output = String::new();
    let mut formatter = fmt::Formatter::new(&mut output);
    let _ = state.fmt(&mut formatter);
}

#[test]
fn test_fmt_with_eoi() {
    let state_id = StateID(1);
    let input_ranges = &[1, 1]; // Single transition where start == end
    let next = &[1u8; 8]; // Ensure we have a valid transition (not DEAD)
    let pattern_ids = &[];
    let accel = &[];

    let state = State {
        id: state_id,
        is_match: false,
        ntrans: 1,
        input_ranges,
        next,
        pattern_ids,
        accel,
    };
    let eoi = StateID(2); // Ensure EOI is not DEAD
    assert_eq!(state.next_at(0), eoi);

    let mut output = String::new();
    let mut formatter = fmt::Formatter::new(&mut output);
    let _ = state.fmt(&mut formatter);
}

