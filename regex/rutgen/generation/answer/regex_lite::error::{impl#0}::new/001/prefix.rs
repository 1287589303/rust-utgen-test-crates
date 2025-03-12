// Answer 0

#[test]
fn test_error_new_empty_string() {
    let error = Error::new("");
}

#[test]
fn test_error_new_general_message() {
    let error = Error::new("error message");
}

#[test]
fn test_error_new_specific_error() {
    let error = Error::new("a specific error");
}

#[test]
fn test_error_new_single_character() {
    let error = Error::new("a");
}

#[test]
fn test_error_new_long_message() {
    let error = Error::new("this is a very long error message that should still be valid.");
}

