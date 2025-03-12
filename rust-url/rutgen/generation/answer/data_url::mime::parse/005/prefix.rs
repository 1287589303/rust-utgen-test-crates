// Answer 0

#[test]
fn test_parse_valid_mime_type_with_parameters() {
    let input = "/example/type; parameter=\"value\"; another=param";
    let result = parse(input);
}

#[test]
fn test_parse_valid_mime_type_with_multiple_parameters() {
    let input = "/another/type; param1=\"value1\"; param2=\"value2\"";
    let result = parse(input);
}

#[test]
fn test_parse_valid_mime_type_with_space_around_parameters() {
    let input = "/example/type;    parameter=\"value\"; another=param  ";
    let result = parse(input);
}

#[test]
fn test_parse_valid_mime_type_with_no_parameters() {
    let input = "/example/type";
    let result = parse(input);
}

