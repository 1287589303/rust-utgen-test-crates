// Answer 0

#[test]
fn test_parser_valid_state() {
    struct MockParser;

    let mock_parser = MockParser;
    let pattern = "a(b|c)*d";
    let parser_i = ParserI::new(&mock_parser, pattern);
    let _result = parser_i.parser();
}

#[test]
fn test_parser_non_null_reference() {
    struct MockParser;

    let mock_parser = MockParser;
    let pattern = "x(y|z)+";
    let parser_i = ParserI::new(&mock_parser, pattern);
    let _result = parser_i.parser();
}

