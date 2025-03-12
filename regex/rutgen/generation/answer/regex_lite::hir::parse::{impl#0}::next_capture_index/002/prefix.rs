// Answer 0

#[test]
fn test_next_capture_index_zero() {
    let config = Config { nest_limit: 10, flags: Flags::default() };
    let pattern = "a";
    let mut parser = Parser::new(config, pattern);
    parser.capture_index.set(0);
    let result = parser.next_capture_index();
}

#[test]
fn test_next_capture_index_max_minus_one() {
    let config = Config { nest_limit: 10, flags: Flags::default() };
    let pattern = "a";
    let mut parser = Parser::new(config, pattern);
    parser.capture_index.set(u32::MAX - 1);
    let result = parser.next_capture_index();
}

#[test]
fn test_next_capture_index_mid_range() {
    let config = Config { nest_limit: 10, flags: Flags::default() };
    let pattern = "a";
    let mut parser = Parser::new(config, pattern);
    parser.capture_index.set(1234);
    let result = parser.next_capture_index();
}

