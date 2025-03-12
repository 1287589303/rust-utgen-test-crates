// Answer 0

#[test]
fn test_ends_in_a_number_with_digit_after_period() {
    let input = "test.5";
    ends_in_a_number(input);
}

#[test]
fn test_ends_in_a_number_with_valid_ipv4_number() {
    let input = "example.com.192";
    ends_in_a_number(input);
}

#[test]
fn test_ends_in_a_number_with_hex_ipv4_number() {
    let input = "sample.0x1A";
    ends_in_a_number(input);
}

#[test]
fn test_ends_in_a_number_with_empty_segments() {
    let input = "hello..123";
    ends_in_a_number(input);
}

#[test]
fn test_ends_in_a_number_with_no_valid_last_segment() {
    let input = "foo.bar.baz";
    ends_in_a_number(input);
}

