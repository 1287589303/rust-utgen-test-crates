// Answer 0

#[test]
fn test_start_pattern_first_call() {
    let mut builder = Builder::new();
    builder.start_pattern().unwrap();
}

#[test]
fn test_start_pattern_second_call() {
    let mut builder = Builder::new();
    builder.start_pattern().unwrap();
    builder.finish_pattern(StateID::ZERO).unwrap();
    builder.start_pattern().unwrap();
}

#[test]
fn test_start_pattern_with_boundaries() {
    let mut builder = Builder::new();
    for i in 0..=MAX_PATTERN_ID {
        builder.start_pattern().unwrap();
        builder.finish_pattern(StateID::ZERO).unwrap();
    }
}

#[test]
#[should_panic]
fn test_start_pattern_without_finish() {
    let mut builder = Builder::new();
    builder.start_pattern().unwrap();
    builder.start_pattern();
}

#[test]
fn test_start_pattern_repeatability() {
    let mut builder = Builder::new();
    for _ in 0..5 {
        builder.start_pattern().unwrap();
        builder.finish_pattern(StateID::ZERO).unwrap();
    }
}

