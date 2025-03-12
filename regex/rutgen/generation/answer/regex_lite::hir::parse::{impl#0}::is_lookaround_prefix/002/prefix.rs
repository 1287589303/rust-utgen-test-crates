// Answer 0

#[test]
fn test_is_lookaround_prefix_1() {
    let config = Config { nest_limit: 10, flags: Flags::default() };
    let pattern = "?!abc";
    let parser = Parser::new(config, pattern);
    parser.is_lookaround_prefix();
}

#[test]
fn test_is_lookaround_prefix_2() {
    let config = Config { nest_limit: 10, flags: Flags::default() };
    let pattern = "?!";
    let parser = Parser::new(config, pattern);
    parser.is_lookaround_prefix();
}

#[test]
fn test_is_lookaround_prefix_3() {
    let config = Config { nest_limit: 10, flags: Flags::default() };
    let pattern = "?<=";
    let parser = Parser::new(config, pattern);
    parser.is_lookaround_prefix();
}

#[test]
fn test_is_lookaround_prefix_4() {
    let config = Config { nest_limit: 10, flags: Flags::default() };
    let pattern = "?<!xyz";
    let parser = Parser::new(config, pattern);
    parser.is_lookaround_prefix();
}

