// Answer 0

#[test]
fn test_fmt_dense_empty() {
    let dense = DenseTransitions {
        transitions: Box::new([]),
    };
    let state = State::Dense(dense);
    let mut output = String::new();
    let _ = write!(&mut output, "{:?}", state);
}

#[test]
fn test_fmt_dense_single_transition() {
    let dense = DenseTransitions {
        transitions: Box::new([StateID(SmallIndex::new_unchecked(1))]),
    };
    let state = State::Dense(dense);
    let mut output = String::new();
    let _ = write!(&mut output, "{:?}", state);
}

