// Answer 0

#[test]
#[should_panic]
fn test_start_pattern_when_pattern_id_is_some() {
    let mut builder = Builder::new();
    builder.pattern_id = Some(PatternID::default());
    let _ = builder.start_pattern();
}

#[test]
#[should_panic]
fn test_start_pattern_when_pattern_id_is_set() {
    let mut builder = Builder::new();
    builder.pattern_id = Some(PatternID::default());
    let _ = builder.start_pattern();
}

#[test]
#[should_panic]
fn test_start_pattern_with_non_none_pattern_id() {
    let mut builder = Builder::new();
    builder.pattern_id = Some(PatternID::default());
    let _ = builder.start_pattern();  
}

