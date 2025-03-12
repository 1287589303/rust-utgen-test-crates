// Answer 0

#[test]
fn test_parse_valid_http_mime() {
    let input = "text/html; charset=UTF-8";
    let result = parse(input);
}

#[test]
fn test_parse_valid_http_mime_with_spaces() {
    let input = "  text/html ; charset=UTF-8  ";
    let result = parse(input);
}

#[test]
fn test_parse_valid_http_mime_without_parameters() {
    let input = "application/json";
    let result = parse(input);
}

#[test]
fn test_parse_valid_http_mime_invalid_subtype() {
    let input = "application/ ; charset=UTF-8";
    let result = parse(input);
}

#[test]
fn test_parse_valid_http_mime_without_type() {
    let input = "/json; charset=UTF-8";
    let result = parse(input);
}

#[test]
fn test_parse_valid_http_mime_missing_subtype() {
    let input = "text/; charset=UTF-8";
    let result = parse(input);
}

