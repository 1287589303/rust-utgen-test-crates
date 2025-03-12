// Answer 0

#[test]
fn test_start_pattern_with_none_pattern_id_and_exceeding_max_patterns() {
    let mut builder = Builder::default();
    builder.start_pattern().unwrap(); // Start a pattern normally

    // Simulate reaching the max patterns supported
    for _ in 0..(u32::MAX as usize) {
        let _ = builder.start_pattern();
    }

    // Now we attempt to start a new pattern which should fail
    let result = builder.start_pattern();
    assert!(result.is_err());
}

#[test]
#[should_panic(expected = "must call 'finish_pattern' first")]
fn test_start_pattern_when_another_pattern_is_active() {
    let mut builder = Builder::default();
    let _ = builder.start_pattern(); // Start a pattern

    // Attempt to start another pattern without finishing the first one
    builder.start_pattern().unwrap(); // This should panic
}

