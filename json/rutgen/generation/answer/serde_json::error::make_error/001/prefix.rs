// Answer 0

#[test]
fn test_make_error_valid_message_with_line_and_column() {
    let msg = String::from("Error occurred at line 5 column 10");
    let error = make_error(msg);
}

#[test]
fn test_make_error_valid_message_with_line_only() {
    let msg = String::from("Error occurred at line 5");
    let error = make_error(msg);
}

#[test]
fn test_make_error_valid_message_with_column_only() {
    let msg = String::from("Error occurred column 10");
    let error = make_error(msg);
}

#[test]
fn test_make_error_valid_message_with_extra_whitespace() {
    let msg = String::from("   Error occurred   at line 5    column 10   ");
    let error = make_error(msg);
}

#[test]
fn test_make_error_message_without_line_and_column() {
    let msg = String::from("General error occurred");
    let error = make_error(msg);
}

#[test]
fn test_make_error_message_with_invalid_line_and_column() {
    let msg = String::from("Error occurred at line X column Y");
    let error = make_error(msg);
}

#[test]
fn test_make_error_empty_message() {
    let msg = String::from("");
    let error = make_error(msg);
}

#[test]
fn test_make_error_message_missing_column() {
    let msg = String::from("Error occurred at line 5 column ");
    let error = make_error(msg);
}

#[test]
fn test_make_error_message_missing_line() {
    let msg = String::from("Error occurred at line  column 10");
    let error = make_error(msg);
}

#[test]
fn test_make_error_message_with_non_digit_characters() {
    let msg = String::from("Error occurred at line 5a column 10b");
    let error = make_error(msg);
}

