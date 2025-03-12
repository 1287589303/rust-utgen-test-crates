// Answer 0

#[test]
fn test_remove_base64_suffix_no_suffix() {
    let input = "data:text/plain;base64,abcdefg";
    let result = remove_base64_suffix(input);
}

#[test]
fn test_remove_base64_suffix_incomplete_suffix() {
    let input = "data:text/plain;base64,4f";
    let result = remove_base64_suffix(input);
}

#[test]
fn test_remove_base64_suffix_with_spaces() {
    let input = "data:text/plain;base64, 645";
    let result = remove_base64_suffix(input);
}

#[test]
fn test_remove_base64_suffix_too_short() {
    let input = "short";
    let result = remove_base64_suffix(input);
}

#[test]
fn test_remove_base64_suffix_incorrect_string_format() {
    let input = "data:text/plain;base64,4s5e6a;"; // incorrect base64 format
    let result = remove_base64_suffix(input);
}

#[test]
fn test_remove_base64_suffix_excess_whitespace() {
    let input = "data:text/plain;base64,   6E  "; // excessive whitespace before semicolon
    let result = remove_base64_suffix(input);
}

