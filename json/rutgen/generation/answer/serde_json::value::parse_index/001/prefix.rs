// Answer 0

#[test]
fn test_parse_index_with_plus_sign_1() {
    let input = "+1";
    let result = parse_index(input);
}

#[test]
fn test_parse_index_with_plus_sign_2() {
    let input = "+100";
    let result = parse_index(input);
}

#[test]
fn test_parse_index_with_plus_sign_empty() {
    let input = "+";
    let result = parse_index(input);
}

