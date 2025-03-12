// Answer 0

#[test]
fn test_memory_usage_sparse_with_one_transition() {
    let transitions = vec![Transition { byte: 0x61, next: StateID(SmallIndex(0)) }];
    let state = State::Sparse { transitions };
    let _ = state.memory_usage();
}

#[test]
fn test_memory_usage_sparse_with_multiple_transitions() {
    let transitions = vec![
        Transition { byte: 0x61, next: StateID(SmallIndex(0)) },
        Transition { byte: 0x62, next: StateID(SmallIndex(1)) },
    ];
    let state = State::Sparse { transitions };
    let _ = state.memory_usage();
}

#[test]
fn test_memory_usage_sparse_with_large_number_of_transitions() {
    let transitions = (0..1000)
        .map(|i| Transition { byte: i as u8, next: StateID(SmallIndex(i as u32)) })
        .collect::<Vec<_>>();
    let state = State::Sparse { transitions };
    let _ = state.memory_usage();
}

