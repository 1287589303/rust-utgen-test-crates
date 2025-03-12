// Answer 0

#[test]
fn test_always_match_with_capture_start() {
    let mut builder = Builder::new();
    let pid = builder.start_pattern().unwrap();
    let start_id = builder.add_capture_start(StateID::ZERO, 0, None).unwrap();
    let end_id = builder.add_capture_end(StateID::ZERO, 0).unwrap();
    let match_id = builder.add_match().unwrap();
    builder.patch(start_id, end_id).unwrap();
    builder.patch(end_id, match_id).unwrap();
    let _ = builder.finish_pattern(start_id).unwrap();
    let _ = builder.build(start_id, start_id).unwrap();
}

#[test]
fn test_always_match_with_capture_end() {
    let mut builder = Builder::new();
    let pid = builder.start_pattern().unwrap();
    let start_id = builder.add_capture_start(StateID::ZERO, 0, None).unwrap();
    let end_id = builder.add_capture_end(StateID::ZERO, 0).unwrap();
    let match_id = builder.add_match().unwrap();
    builder.patch(start_id, end_id).unwrap();
    builder.patch(end_id, match_id).unwrap();
    let _ = builder.finish_pattern(start_id).unwrap();
    let _ = builder.build(start_id, start_id).unwrap();
}

#[test]
fn test_always_match_with_utf8() {
    let mut builder = Builder::new();
    builder.set_utf8(true);
    let pid = builder.start_pattern().unwrap();
    let start_id = builder.add_capture_start(StateID::ZERO, 0, None).unwrap();
    let end_id = builder.add_capture_end(StateID::ZERO, 0).unwrap();
    let match_id = builder.add_match().unwrap();
    builder.patch(start_id, end_id).unwrap();
    builder.patch(end_id, match_id).unwrap();
    let _ = builder.finish_pattern(start_id).unwrap();
    let _ = builder.build(start_id, start_id).unwrap();
}

#[test]
fn test_always_match_with_reverse() {
    let mut builder = Builder::new();
    builder.set_reverse(true);
    let pid = builder.start_pattern().unwrap();
    let start_id = builder.add_capture_start(StateID::ZERO, 0, None).unwrap();
    let end_id = builder.add_capture_end(StateID::ZERO, 0).unwrap();
    let match_id = builder.add_match().unwrap();
    builder.patch(start_id, end_id).unwrap();
    builder.patch(end_id, match_id).unwrap();
    let _ = builder.finish_pattern(start_id).unwrap();
    let _ = builder.build(start_id, start_id).unwrap();
}

