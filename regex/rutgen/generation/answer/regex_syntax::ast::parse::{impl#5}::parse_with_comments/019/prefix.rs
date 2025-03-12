// Answer 0

#[test]
fn test_parse_with_comments_valid_case() {
    let pattern = "a*b*";
    let parser = Parser::new();
    parser.reset(); // initialize parser state
    let parser_i = ParserI {
        parser: &parser,
        pattern,
    };

    let result = parser_i.parse_with_comments();
    let _ = result.unwrap();
}

#[test]
fn test_parse_with_comments_unclosed_repetition() {
    let pattern = "a{1,2*"; // malformed input
    let parser = Parser::new();
    parser.reset(); // initialize parser state
    let parser_i = ParserI {
        parser: &parser,
        pattern,
    };

    let result = parser_i.parse_with_comments();
    let _ = result.unwrap_err();
}

#[test]
fn test_parse_with_comments_nesting_exceedance() {
    let pattern = "((a|b)*c)"; // valid inputs but nested
    let parser = Parser::new();
    parser.reset(); // initialize parser state
    let parser_i = ParserI {
        parser: &parser,
        pattern,
    };

    let result = parser_i.parse_with_comments();
    let _ = result.unwrap();
}

