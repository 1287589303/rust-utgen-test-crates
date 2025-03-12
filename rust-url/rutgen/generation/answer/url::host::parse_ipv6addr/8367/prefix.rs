// Answer 0

#[test]
fn test_parse_ipv6addr_case1() {
    let input = "::1";
    let result = parse_ipv6addr(input);
}

#[test]
fn test_parse_ipv6addr_case2() {
    let input = "::1.0.0.1";
    let result = parse_ipv6addr(input);
}

#[test]
fn test_parse_ipv6addr_case3() {
    let input = "::0.0.0.0";
    let result = parse_ipv6addr(input);
}

#[test]
fn test_parse_ipv6addr_case4() {
    let input = "::ffff:192.168.1.1";
    let result = parse_ipv6addr(input);
}

