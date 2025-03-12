// Answer 0

#[test]
fn test_finish_pattern_valid_case() {
    let mut builder = Builder::new();
    let start_id = builder.start_pattern().unwrap();
    builder.finish_pattern(start_id).unwrap();
}

#[test]
fn test_finish_pattern_multiple_patterns() {
    let mut builder = Builder::new();
    let start_id1 = builder.start_pattern().unwrap();
    let pid1 = builder.finish_pattern(start_id1).unwrap();

    let start_id2 = builder.start_pattern().unwrap();
    let pid2 = builder.finish_pattern(start_id2).unwrap();

    assert!(pid2 == PatternID(SmallIndex(1)));
}

#[test]
#[should_panic]
fn test_finish_pattern_no_start_pattern() {
    let mut builder = Builder::new();
    let start_id = StateID(SmallIndex(0));
    builder.finish_pattern(start_id).unwrap();
}

#[test]
fn test_finish_pattern_with_valid_start_id() {
    let mut builder = Builder::new();
    let start_id = builder.start_pattern().unwrap();
    let state_id = builder.add_empty().unwrap();
    builder.finish_pattern(state_id).unwrap();
}

