// Answer 0

#[test]
fn test_escape_utf8_with_valid_zero_sequence() {
    let mut repr = String::new();
    escape_utf8("Hello\0World0", &mut repr);
}

#[test]
fn test_escape_utf8_with_multiple_zero_sequences() {
    let mut repr = String::new();
    escape_utf8("Test\0Example1\0Another2", &mut repr);
}

#[test]
fn test_escape_utf8_with_zero_followed_by_eight() {
    let mut repr = String::new();
    escape_utf8("Start\0End8", &mut repr);
}

#[test]
fn test_escape_utf8_with_zero_followed_by_non_digit() {
    let mut repr = String::new();
    escape_utf8("Data\0CheckA", &mut repr);
}

#[test]
fn test_escape_utf8_with_zero_at_end() {
    let mut repr = String::new();
    escape_utf8("Sample\0", &mut repr);
}

