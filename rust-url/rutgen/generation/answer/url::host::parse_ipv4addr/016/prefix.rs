// Answer 0

#[test]
fn test_parse_ipv4addr_valid_case_1() {
    let input = "192.168.1.1";
    let result = parse_ipv4addr(input);
}

#[test]
fn test_parse_ipv4addr_valid_case_2() {
    let input = "0.0.0.0";
    let result = parse_ipv4addr(input);
}

#[test]
fn test_parse_ipv4addr_valid_case_3() {
    let input = "255.255.255.255";
    let result = parse_ipv4addr(input);
}

#[test]
fn test_parse_ipv4addr_with_leading_zeros() {
    let input = "001.002.003.004";
    let result = parse_ipv4addr(input);
}

#[test]
fn test_parse_ipv4addr_invalid_case_too_many_parts() {
    let input = "192.168.1.1.2";
    let result = parse_ipv4addr(input);
}

#[test]
fn test_parse_ipv4addr_invalid_case_empty_part() {
    let input = "192.168..1";
    let result = parse_ipv4addr(input);
}

#[test]
fn test_parse_ipv4addr_invalid_case_non_numeric_part() {
    let input = "192.168.one.1";
    let result = parse_ipv4addr(input);
}

#[test]
fn test_parse_ipv4addr_invalid_case_out_of_range() {
    let input = "256.256.256.256";
    let result = parse_ipv4addr(input);
}

