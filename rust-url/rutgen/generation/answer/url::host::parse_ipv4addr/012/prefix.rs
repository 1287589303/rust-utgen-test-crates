// Answer 0

#[test]
fn test_parse_ipv4addr_too_many_parts() {
    let input = "192.168.0.1.1";
    let _ = parse_ipv4addr(input);
}

#[test]
fn test_parse_ipv4addr_too_many_parts_with_repeating_numbers() {
    let input = "256.256.256.256.256";
    let _ = parse_ipv4addr(input);
}

#[test]
fn test_parse_ipv4addr_too_many_parts_with_letters() {
    let input = "1.2.3.4.5a";
    let _ = parse_ipv4addr(input);
}

#[test]
fn test_parse_ipv4addr_too_many_parts_with_empty_segment() {
    let input = "1.2.3.4.";
    let _ = parse_ipv4addr(input);
}

#[test]
fn test_parse_ipv4addr_too_many_parts_with_multiple_empty_segments() {
    let input = "1..2.3.4.5";
    let _ = parse_ipv4addr(input);
}

