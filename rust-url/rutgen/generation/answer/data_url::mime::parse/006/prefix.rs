// Answer 0

#[test]
fn test_parse_empty_string() {
    let input = "";
    let result = parse(input);
}

#[test]
fn test_parse_only_whitespace() {
    let input = "   ";
    let result = parse(input);
}

#[test]
fn test_parse_missing_subtype() {
    let input = "text/;param=value";
    let result = parse(input);
}

#[test]
fn test_parse_invalid_type_double_slash() {
    let input = "text//param=value";
    let result = parse(input);
}

#[test]
fn test_parse_invalid_subtype() {
    let input = "text/invalid^subtype;param=value";
    let result = parse(input);
}

