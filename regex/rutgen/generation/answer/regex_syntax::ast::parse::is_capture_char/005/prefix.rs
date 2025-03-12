// Answer 0

#[test]
fn test_is_capture_char_valid_underscore() {
    let c: char = '_';
    let first: bool = false;
    let result = is_capture_char(c, first);
}

#[test]
fn test_is_capture_char_valid_dot() {
    let c: char = '.';
    let first: bool = false;
    let result = is_capture_char(c, first);
}

#[test]
fn test_is_capture_char_invalid_square_bracket() {
    let c: char = '[';
    let first: bool = false;
    let result = is_capture_char(c, first);
}

