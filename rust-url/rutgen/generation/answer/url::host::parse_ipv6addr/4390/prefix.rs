// Answer 0

#[test]
fn test_parse_ipv6addr_valid_input() {
    let input = "::1.0.0.1";
    let result = parse_ipv6addr(input);
}

#[test]
fn test_parse_ipv6addr_valid_input_with_compression() {
    let input = "::ffff:192.168.1.1";
    let result = parse_ipv6addr(input);
}

#[test]
fn test_parse_ipv6addr_valid_input_with_two_separators() {
    let input = "::1.1.1.1";
    let result = parse_ipv6addr(input);
}

#[test]
fn test_parse_ipv6addr_valid_input_with_leading_zeroes() {
    let input = "::01.01.01.01";
    let result = parse_ipv6addr(input);
}

