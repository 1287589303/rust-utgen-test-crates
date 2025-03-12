// Answer 0

#[test]
fn test_parse_index_zero_length_greater_than_one() {
    let input = "00";
    let result = parse_index(input);
}

#[test]
fn test_parse_index_zero_leading_digits() {
    let input = "0123";
    let result = parse_index(input);
}

#[test]
fn test_parse_index_zero_with_letters() {
    let input = "0abc";
    let result = parse_index(input);
}

#[test]
fn test_parse_index_zero_decimal() {
    let input = "0.1";
    let result = parse_index(input);
}

#[test]
fn test_parse_index_zero_longer_digits() {
    let input = "000456";
    let result = parse_index(input);
}

