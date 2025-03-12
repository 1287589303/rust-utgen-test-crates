// Answer 0

#[test]
fn test_parse_ipv6addr_valid_case_1() {
    let input = "::1.1.1.1";
    let result = parse_ipv6addr(input);
}

#[test]
fn test_parse_ipv6addr_valid_case_2() {
    let input = "2001:0db8:85a3:0000:0000:8a2e:0370:7334";
    let result = parse_ipv6addr(input);
}

#[test]
fn test_parse_ipv6addr_valid_case_3() {
    let input = "2001:db8::ff00:42:8329";
    let result = parse_ipv6addr(input);
}

#[test]
fn test_parse_ipv6addr_valid_case_4() {
    let input = "::ffff:192.168.1.1";
    let result = parse_ipv6addr(input);
}

#[test]
fn test_parse_ipv6addr_valid_case_5() {
    let input = "fe80::1ff:fe23:4567:890a";
    let result = parse_ipv6addr(input);
}

