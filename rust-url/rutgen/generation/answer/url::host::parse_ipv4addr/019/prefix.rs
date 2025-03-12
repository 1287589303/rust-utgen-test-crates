// Answer 0

#[test]
fn test_ipv4_invalid_too_many_parts() {
    let input = "1.2.3.4.5";
    let _result = parse_ipv4addr(input);
}

#[test]
fn test_ipv4_invalid_part_too_large() {
    let input = "256.256.256.256";
    let _result = parse_ipv4addr(input);
}

#[test]
fn test_ipv4_invalid_not_enough_parts() {
    let input = "1.2.3";
    let _result = parse_ipv4addr(input);
}

#[test]
fn test_ipv4_invalid_last_part_too_large() {
    let input = "1.2.3.256";
    let _result = parse_ipv4addr(input);
}

