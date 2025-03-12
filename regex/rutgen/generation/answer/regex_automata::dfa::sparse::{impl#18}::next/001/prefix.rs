// Answer 0

#[cfg(test)]
fn test_next_with_start_bound() {
    let input = 5;
    let input_ranges = &[5, 10]; // inclusive range where start == input
    let next_states = &[0, 1]; // dummy state transitions
    let state = State {
        id: StateID::default(),
        is_match: false,
        ntrans: 2,
        input_ranges,
        next: next_states,
        pattern_ids: &[],
        accel: &[],
    };
    let _result = state.next(input);
}

#[cfg(test)]
fn test_next_with_end_bound() {
    let input = 10;
    let input_ranges = &[5, 10]; // inclusive range where input == end
    let next_states = &[1, 2]; // dummy state transitions
    let state = State {
        id: StateID::default(),
        is_match: false,
        ntrans: 2,
        input_ranges,
        next: next_states,
        pattern_ids: &[],
        accel: &[],
    };
    let _result = state.next(input);
}

