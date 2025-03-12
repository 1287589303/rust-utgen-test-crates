// Answer 0

#[test]
fn test_parse_ipv6addr_case_1() {
    let input = ":1:2:3:4:5:6:7:8";
    let result = parse_ipv6addr(input);
}

#[test]
fn test_parse_ipv6addr_case_2() {
    let input = ":abcd:ef12:3456:789a:bcde:f012:3456:789a";
    let result = parse_ipv6addr(input);
}

#[test]
fn test_parse_ipv6addr_case_3() {
    let input = ":0:0:0:0:0:0:0:0";
    let result = parse_ipv6addr(input);
}

#[test]
fn test_parse_ipv6addr_case_4() {
    let input = ":ffff:ffff:ffff:ffff:ffff:ffff:ffff:ffff";
    let result = parse_ipv6addr(input);
}

