// Answer 0

#[test]
fn test_parse_opaque_valid_ipv6_address_1() {
    let input = "[::1]";
    let _ = Host::parse_opaque(input);
}

#[test]
fn test_parse_opaque_valid_ipv6_address_2() {
    let input = "[2001:0db8:85a3:0000:0000:8a2e:0370:7334]";
    let _ = Host::parse_opaque(input);
}

#[test]
fn test_parse_opaque_valid_ipv6_address_with_compression() {
    let input = "[2001:db8::1]";
    let _ = Host::parse_opaque(input);
}

#[test]
fn test_parse_opaque_valid_ipv6_address_with_mixed_zeroes() {
    let input = "[2001:0db8:0:0:0:0:0:2]";
    let _ = Host::parse_opaque(input);
}

