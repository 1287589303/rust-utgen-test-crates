// Answer 0

#[test]
fn test_parse_ipv6_invalid_missing_closing_bracket() {
    let input = "[192.168.1.1"; // input starts with '[' and does not end with ']'
    let _result = Host::parse(input);
}

#[test]
fn test_parse_ipv6_invalid_partial_brackets() {
    let input = "[::1"; // input starts with '[' and does not end with ']'
    let _result = Host::parse(input);
}

#[test]
fn test_parse_ipv6_invalid_non_ending_bracket() {
    let input = "[some:ipv6:address"; // input starts with '[' and does not end with ']'
    let _result = Host::parse(input);
}

