// Answer 0

#[test]
fn test_parse_ipv6addr_empty_string() {
    let input = "";
    parse_ipv6addr(input);
}

#[test]
fn test_parse_ipv6addr_single_character() {
    let input = "a";
    parse_ipv6addr(input);
}

#[test]
fn test_parse_ipv6addr_single_colon() {
    let input = ":";
    parse_ipv6addr(input);
}

