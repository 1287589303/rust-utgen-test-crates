// Answer 0

#[test]
fn test_memory_usage_splits_empty_targets() {
    let state = State::Splits {
        targets: vec![],
        reverse: false,
    };
    let _ = state.memory_usage();
}

#[test]
fn test_memory_usage_splits_single_target() {
    let state = State::Splits {
        targets: vec![0],
        reverse: true,
    };
    let _ = state.memory_usage();
}

#[test]
fn test_memory_usage_splits_multiple_targets() {
    let state = State::Splits {
        targets: vec![0, 1, 2],
        reverse: false,
    };
    let _ = state.memory_usage();
}

#[test]
fn test_memory_usage_splits_ten_targets() {
    let state = State::Splits {
        targets: vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
        reverse: true,
    };
    let _ = state.memory_usage();
}

#[test]
fn test_memory_usage_splits_hundred_targets() {
    let state = State::Splits {
        targets: (0..100).collect(),
        reverse: false,
    };
    let _ = state.memory_usage();
}

#[test]
fn test_memory_usage_splits_thousand_targets() {
    let state = State::Splits {
        targets: (0..1000).collect(),
        reverse: true,
    };
    let _ = state.memory_usage();
}

