// Answer 0

#[test]
fn test_parse_header_case_with_control_chars() {
    let input = "\t\n\rhello\x00world\x1F";
    let result = parse_header(input);
}

#[test]
fn test_parse_header_with_space_and_quotes() {
    let input = "hello world \"quoted\" <tag>";
    let result = parse_header(input);
}

#[test]
fn test_parse_header_with_question_mark() {
    let input = "query?value";
    let result = parse_header(input);
}

#[test]
fn test_parse_header_with_mixed_characters() {
    let input = "example: \t\n\r test\x7F value";
    let result = parse_header(input);
}

#[test]
fn test_parse_header_with_semi_colon_prefix() {
    let input = ";application/json";
    let result = parse_header(input);
}

#[test]
fn test_parse_header_with_valid_mime_type() {
    let input = "image/jpeg";
    let result = parse_header(input);
}

#[test]
fn test_parse_header_empty_string() {
    let input = "";
    let result = parse_header(input);
}

