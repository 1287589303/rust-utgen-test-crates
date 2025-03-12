// Answer 0

#[test]
fn test_add_capture_start_with_new_pattern_and_new_group_index() {
    let mut builder = Builder::new();
    let _ = builder.start_pattern();
    let pid = builder.current_pattern_id();
    let _ = builder.add_capture_start(StateID::ZERO, 0, None).unwrap(); // initial unnamed capture
    let _ = builder.add_capture_start(StateID::ZERO, 1, Some(Arc::from("group1"))).unwrap(); // first named capture
    let _ = builder.add_capture_start(StateID::ZERO, 2, Some(Arc::from("group2"))).unwrap(); // second named capture
    let result = builder.finish_pattern(StateID::ZERO);
}

#[test]
fn test_add_capture_start_with_existing_pattern_and_sequential_group_index() {
    let mut builder = Builder::new();
    let _ = builder.start_pattern();
    let pid = builder.current_pattern_id();
    let _ = builder.add_capture_start(StateID::ZERO, 0, None).unwrap();
    let _ = builder.add_capture_start(StateID::ZERO, 1, Some(Arc::from("group1"))).unwrap();
    let _ = builder.finish_pattern(StateID::ZERO);
    let _ = builder.start_pattern(); // start new pattern
    let _ = builder.add_capture_start(StateID::ZERO, 0, None).unwrap();
    let _ = builder.add_capture_start(StateID::ZERO, 1, Some(Arc::from("group1"))).unwrap(); // reuse group index
    let result = builder.build(StateID::ZERO, StateID::ZERO);
}

#[test]
fn test_add_capture_start_with_disallowed_duplicate_group_name() {
    let mut builder = Builder::new();
    let _ = builder.start_pattern();
    let pid = builder.current_pattern_id();
    let _ = builder.add_capture_start(StateID::ZERO, 0, None).unwrap();
    let _ = builder.add_capture_start(StateID::ZERO, 1, Some(Arc::from("group1"))).unwrap();
    let _ = builder.add_capture_start(StateID::ZERO, 1, Some(Arc::from("group1"))).unwrap(); // duplicate name
    let _ = builder.finish_pattern(StateID::ZERO).unwrap();
    let result = builder.build(StateID::ZERO, StateID::ZERO);
} 

#[test]
fn test_add_capture_start_with_multiple_patterns() {
    let mut builder = Builder::new();
    let _ = builder.start_pattern();
    let pid = builder.current_pattern_id();
    let _ = builder.add_capture_start(StateID::ZERO, 0, None).unwrap();
    let _ = builder.add_capture_start(StateID::ZERO, 1, Some(Arc::from("group1"))).unwrap();
    let _ = builder.finish_pattern(StateID::ZERO);
    let _ = builder.start_pattern(); // start a new pattern
    let second_pid = builder.current_pattern_id();
    let _ = builder.add_capture_start(StateID::ZERO, 0, None).unwrap();
    let _ = builder.add_capture_start(StateID::ZERO, 1, Some(Arc::from("group2"))).unwrap();
    let _ = builder.finish_pattern(StateID::ZERO);
    let result = builder.build(StateID::ZERO, StateID::ZERO);
}

