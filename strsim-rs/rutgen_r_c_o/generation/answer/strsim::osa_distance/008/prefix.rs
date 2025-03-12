// Answer 0

#[test]
fn test_osa_distance_no_common_chars_equal_length() {
    let a = "abcdefghij";
    let b = "klmnopqrst";
    let result = osa_distance(a, b);
}

#[test]
fn test_osa_distance_no_common_chars_equal_length_boundary() {
    let a = "abcdefghij";
    let b = "ABCDEFGHIJK"; // Testing with a character set greater than 10 to ensure correct handling.
    let result = osa_distance(a, b);
}

#[test]
fn test_osa_distance_no_common_chars_equal_length_maximum() {
    let a = "1234567890";
    let b = "!@#$%^&*()"; // Both strings are of maximum length (10) with no common characters.
    let result = osa_distance(a, b);
}

