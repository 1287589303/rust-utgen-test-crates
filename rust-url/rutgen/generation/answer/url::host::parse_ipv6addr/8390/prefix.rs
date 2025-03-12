// Answer 0

#[test]
fn test_parse_ipv6addr_case_1() {
    let input = "::1";
    let result = parse_ipv6addr(input);
}

#[test]
fn test_parse_ipv6addr_case_2() {
    let input = "::0:0:0:0:0:1";
    let result = parse_ipv6addr(input);
}

#[test]
fn test_parse_ipv6addr_case_3() {
    let input = "::ffff:192.168.1.1";
    let result = parse_ipv6addr(input);
}

#[test]
fn test_parse_ipv6addr_case_4() {
    let input = "::1234:5678:9abc:def0:1234:5678:9abc:def0";
    let result = parse_ipv6addr(input);
}

