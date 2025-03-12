// Answer 0

#[test]
fn test_add_range_valid_transition() {
    let mut builder = Builder::new();
    let trans = Transition { start: 0, end: 10, next: StateID::default() };
    let _ = builder.add_range(trans);
}

#[test]
fn test_add_range_boundary_transition_start_min() {
    let mut builder = Builder::new();
    let trans = Transition { start: 0, end: 255, next: StateID::default() };
    let _ = builder.add_range(trans);
}

#[test]
fn test_add_range_boundary_transition_end_max() {
    let mut builder = Builder::new();
    let trans = Transition { start: 255, end: 255, next: StateID::default() };
    let _ = builder.add_range(trans);
}

#[test]
fn test_add_range_invalid_transition_end_less_than_start() {
    let mut builder = Builder::new();
    let trans = Transition { start: 10, end: 5, next: StateID::default() };
    let result = builder.add_range(trans);
    assert!(result.is_err());
}

#[test]
fn test_add_range_exceed_state_limit() {
    let mut builder = Builder::new();
    builder.states = vec![State::ByteRange { trans: Transition { start: 0, end: 0, next: StateID::default() } }; 255];
    let trans = Transition { start: 0, end: 10, next: StateID::default() };
    let result = builder.add_range(trans);
    assert!(result.is_err());
}

#[test]
fn test_add_range_exceed_memory_limit() {
    let mut builder = Builder::new();
    builder.memory_states = 1024; // Assume this exceeds some set limit
    let trans = Transition { start: 0, end: 10, next: StateID::default() };
    let result = builder.add_range(trans);
    assert!(result.is_err());
}

