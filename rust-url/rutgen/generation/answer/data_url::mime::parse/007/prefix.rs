// Answer 0

#[test]
fn test_parse_invalid_type() {
    let input = "invalid%type/subtype";
    let result = parse(input);
}

#[test]
fn test_parse_empty_string() {
    let input = "";
    let result = parse(input);
}

#[test]
fn test_parse_space_before_type() {
    let input = "   /subtype";
    let result = parse(input);
}

#[test]
fn test_parse_space_before_subtype() {
    let input = "type/   ";
    let result = parse(input);
}

#[test]
fn test_parse_invalid_subtype() {
    let input = "type/invalid%subtype";
    let result = parse(input);
}

