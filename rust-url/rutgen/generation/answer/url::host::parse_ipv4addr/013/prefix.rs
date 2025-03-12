// Answer 0

#[test]
fn test_parse_ipv4addr_invalid_address_too_large() {
    let input = "256.256.256.256";
    let _result = parse_ipv4addr(input);
}

#[test]
fn test_parse_ipv4addr_invalid_address_too_many_parts() {
    let input = "192.168.1.1.1";
    let _result = parse_ipv4addr(input);
}

#[test]
fn test_parse_ipv4addr_invalid_address_non_numeric_parts() {
    let input = "abc.def.ghi.jkl";
    let _result = parse_ipv4addr(input);
}

#[test]
fn test_parse_ipv4addr_invalid_address_empty_segment() {
    let input = "192..1";
    let _result = parse_ipv4addr(input);
}

