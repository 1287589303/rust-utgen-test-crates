// Answer 0

#[test]
fn test_add_sparse_no_transitions() {
    let mut builder = Builder::new();
    let pattern_id = builder.start_pattern().unwrap();
    builder.finish_pattern(StateID::default()).unwrap();
    let transitions: Vec<Transition> = Vec::new();
    builder.add_sparse(transitions).unwrap();
}

#[test]
fn test_add_sparse_single_transition() {
    let mut builder = Builder::new();
    let pattern_id = builder.start_pattern().unwrap();
    builder.finish_pattern(StateID::default()).unwrap();
    let transitions = vec![Transition { start: 0, end: 1, next: StateID(SmallIndex::new(1).unwrap()) }];
    builder.add_sparse(transitions).unwrap();
}

#[test]
fn test_add_sparse_multiple_transitions() {
    let mut builder = Builder::new();
    let pattern_id = builder.start_pattern().unwrap();
    builder.finish_pattern(StateID::default()).unwrap();
    let transitions = vec![
        Transition { start: 0, end: 1, next: StateID(SmallIndex::new(1).unwrap()) },
        Transition { start: 2, end: 3, next: StateID(SmallIndex::new(2).unwrap()) },
        Transition { start: 4, end: 5, next: StateID(SmallIndex::new(3).unwrap()) }
    ];
    builder.add_sparse(transitions).unwrap();
}

#[test]
fn test_add_sparse_overlapping_transitions() {
    let mut builder = Builder::new();
    let pattern_id = builder.start_pattern().unwrap();
    builder.finish_pattern(StateID::default()).unwrap();
    let transitions = vec![
        Transition { start: 0, end: 2, next: StateID(SmallIndex::new(1).unwrap()) },
        Transition { start: 1, end: 3, next: StateID(SmallIndex::new(2).unwrap()) }
    ];
    let result = builder.add_sparse(transitions);
    assert!(result.is_err());
}

#[test]
fn test_add_sparse_non_ascending_order() {
    let mut builder = Builder::new();
    let pattern_id = builder.start_pattern().unwrap();
    builder.finish_pattern(StateID::default()).unwrap();
    let transitions = vec![
        Transition { start: 1, end: 2, next: StateID(SmallIndex::new(1).unwrap()) },
        Transition { start: 0, end: 1, next: StateID(SmallIndex::new(2).unwrap()) }
    ];
    let result = builder.add_sparse(transitions);
    assert!(result.is_err());
}

#[test]
fn test_add_sparse_exceeding_size_limit() {
    let mut builder = Builder::new();
    builder.set_size_limit(Some(10)).unwrap();
    let pattern_id = builder.start_pattern().unwrap();
    builder.finish_pattern(StateID::default()).unwrap();
    let transitions = vec![
        Transition { start: 0, end: 255, next: StateID(SmallIndex::new(1).unwrap()) }
    ];
    let result = builder.add_sparse(transitions);
    assert!(result.is_err());
}

