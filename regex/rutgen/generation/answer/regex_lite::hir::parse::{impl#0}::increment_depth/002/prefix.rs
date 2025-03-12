// Answer 0

#[test]
fn test_increment_depth_at_limit() {
    let config = Config {
        nest_limit: 5,
        flags: Flags::default(),
    };
    let pattern = "abc";
    let parser = Parser::new(config.clone(), pattern);
    parser.depth.set(5);

    let result = parser.increment_depth();
}

#[test]
fn test_increment_depth_at_high_limit() {
    let config = Config {
        nest_limit: 10000,
        flags: Flags::default(),
    };
    let pattern = "xyz";
    let parser = Parser::new(config.clone(), pattern);
    parser.depth.set(10000);

    let result = parser.increment_depth();
}

