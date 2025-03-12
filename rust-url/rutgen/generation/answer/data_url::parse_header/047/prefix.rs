// Answer 0

#[test]
fn test_parse_header_semicolon_start() {
    let input = ";charset=utf-8";
    let (mime_type, base64) = parse_header(input);
}

#[test]
fn test_parse_header_semicolon_start_only_printable() {
    let input = ";application/json";
    let (mime_type, base64) = parse_header(input);
}

#[test]
fn test_parse_header_semicolon_start_with_spaces() {
    let input = " ; text/html ";
    let (mime_type, base64) = parse_header(input);
}

#[test]
fn test_parse_header_semicolon_start_with_query() {
    let input = ";image/png?width=400&height=400";
    let (mime_type, base64) = parse_header(input);
}

#[test]
fn test_parse_header_semicolon_start_multiple_parameters() {
    let input = ";text/plain; charset=UTF-8; version=1.0";
    let (mime_type, base64) = parse_header(input);
}

