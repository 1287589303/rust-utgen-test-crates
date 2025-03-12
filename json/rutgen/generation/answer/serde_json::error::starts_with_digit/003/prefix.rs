// Answer 0

#[test]
fn test_starts_with_digit_empty_string() {
    let slice = "";
    let result = starts_with_digit(slice);
}

#[test]
fn test_starts_with_digit_non_empty_string() {
    let slice = "a";
    let result = starts_with_digit(slice);
}

#[test]
fn test_starts_with_digit_digit_string() {
    let slice = "1";
    let result = starts_with_digit(slice);
}

#[test]
fn test_starts_with_digit_spaces_string() {
    let slice = "   ";
    let result = starts_with_digit(slice);
}

#[test]
fn test_starts_with_digit_mixed_string() {
    let slice = "a1";
    let result = starts_with_digit(slice);
}

