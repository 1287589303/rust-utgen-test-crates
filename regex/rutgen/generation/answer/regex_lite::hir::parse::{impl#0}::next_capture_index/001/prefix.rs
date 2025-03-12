// Answer 0

#[test]
fn test_next_capture_index_valid_increment_0() {
    let config = Config { nest_limit: 10, flags: Flags::default() };
    let pattern = "any pattern";
    let parser = Parser::new(config, pattern);
    parser.capture_index.set(0);
    let _ = parser.next_capture_index();
}

#[test]
fn test_next_capture_index_valid_increment_1() {
    let config = Config { nest_limit: 10, flags: Flags::default() };
    let pattern = "any pattern";
    let parser = Parser::new(config, pattern);
    parser.capture_index.set(1);
    let _ = parser.next_capture_index();
}

#[test]
#[should_panic]
fn test_next_capture_index_exceeding_limit() {
    let config = Config { nest_limit: 10, flags: Flags::default() };
    let pattern = "any pattern";
    let parser = Parser::new(config, pattern);
    parser.capture_index.set(u32::MAX);
    let _ = parser.next_capture_index();
}

