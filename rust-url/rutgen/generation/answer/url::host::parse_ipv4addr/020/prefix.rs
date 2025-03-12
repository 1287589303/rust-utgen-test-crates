// Answer 0

#[test]
fn test_parse_ipv4addr_invalid_numbers_exceeding_255() {
    let input = "256.256.256.256";
    let result = parse_ipv4addr(input);
}

#[test]
fn test_parse_ipv4addr_invalid_zero_exceeding() {
    let input = "0.0.0.256";
    let result = parse_ipv4addr(input);
}

#[test]
fn test_parse_ipv4addr_invalid_large_numbers() {
    let input = "300.300.300.300";
    let result = parse_ipv4addr(input);
}

