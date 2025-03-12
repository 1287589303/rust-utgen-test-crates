// Answer 0

#[test]
fn test_current_pattern_id_valid_case() {
    let mut builder = Builder::new();
    let _ = builder.start_pattern().unwrap();
    let pattern_id = builder.current_pattern_id();
}

#[test]
#[should_panic]
fn test_current_pattern_id_without_start_pattern() {
    let builder = Builder::new();
    let _ = builder.current_pattern_id();
}

