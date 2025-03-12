// Answer 0

#[test]
fn test_parse_ipv4addr_invalid_address_overflow() {
    let input = "192.168.1.256.";
    let result = parse_ipv4addr(input);
}

#[test]
fn test_parse_ipv4addr_invalid_address_empty_part() {
    let input = "192.168.1..";
    let result = parse_ipv4addr(input);
}

#[test]
fn test_parse_ipv4addr_invalid_address_extra_part() {
    let input = "192.168.1.1.1";
    let result = parse_ipv4addr(input);
}

#[test]
fn test_parse_ipv4addr_invalid_address_all_parts_overflow() {
    let input = "256.256.256.256";
    let result = parse_ipv4addr(input);
}

