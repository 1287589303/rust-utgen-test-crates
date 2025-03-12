// Answer 0

#[test]
fn test_increment_depth_exceeding_nest_limit_zero() {
    let config = Config { nest_limit: 0, flags: Flags::default() };
    let pattern = "test";
    let parser = Parser::new(config, pattern);
    parser.depth.set(1);
    let _ = parser.increment_depth();
}

#[test]
fn test_increment_depth_exceeding_nest_limit_negative() {
    let config = Config { nest_limit: u32::MAX, flags: Flags::default() };
    let pattern = "test";
    let parser = Parser::new(config, pattern);
    parser.depth.set(u32::MAX);
    let _ = parser.increment_depth();
}

