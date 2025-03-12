// Answer 0

#[test]
fn test_pattern_len_empty() {
    let builder = Builder::new();
    let len = builder.pattern_len();
}

#[test]
fn test_pattern_len_one_pattern() {
    let mut builder = Builder::new();
    let _ = builder.start_pattern();
    let _ = builder.finish_pattern(StateID::default());
    let len = builder.pattern_len();
}

#[test]
fn test_pattern_len_multiple_patterns() {
    let mut builder = Builder::new();
    for _ in 0..10 {
        let _ = builder.start_pattern();
        let _ = builder.finish_pattern(StateID::default());
    }
    let len = builder.pattern_len();
}

