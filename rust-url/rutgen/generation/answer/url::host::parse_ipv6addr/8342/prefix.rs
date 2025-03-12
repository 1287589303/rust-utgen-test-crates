// Answer 0

#[test]
fn test_parse_ipv6addr_case_1() {
    let input = "::1.0.0.0"; // len == 2
    let result = parse_ipv6addr(input); // input[0] == b':', i == len, is_ip_v4 == true, piece_pointer == 6
}

#[test]
fn test_parse_ipv6addr_case_2() {
    let input = "::a:b:c:d:e:f"; // len == 2, input[0] == b':'
    let result = parse_ipv6addr(input); // i == len, is_ip_v4 == true, piece_pointer == 6
}

#[test]
fn test_parse_ipv6addr_case_3() {
    let input = "::1:0.0.0.0"; // len == 2
    let result = parse_ipv6addr(input); // input[0] == b':', i == len, is_ip_v4 == true, piece_pointer == 6
}

#[test]
fn test_parse_ipv6addr_case_4() {
    let input = "::a:b:c:d:e:f:0.0.0.0"; // len == 2
    let result = parse_ipv6addr(input); // input[0] == b':', i == len, is_ip_v4 == true, piece_pointer == 6
}

#[test]
fn test_parse_ipv6addr_case_5() {
    let input = "::1234:.8"; // len == 2
    let result = parse_ipv6addr(input); // input[0] == b':', i == len, is_ip_v4 == true, piece_pointer == 6
}

