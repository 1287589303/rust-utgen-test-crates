// Answer 0

#[test]
fn test_swap_greed_enable() {
    let mut parser_builder = ParserBuilder::new();
    parser_builder.swap_greed(true);
}

#[test]
fn test_swap_greed_disable() {
    let mut parser_builder = ParserBuilder::new();
    parser_builder.swap_greed(false);
}

