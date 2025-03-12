// Answer 0

#[test]
fn test_is_capture_char_with_valid_char_underscore() {
    let c = '_';
    let first = false;
    is_capture_char(c, first);
}

#[test]
fn test_is_capture_char_with_valid_char_open_bracket() {
    let c = '[';
    let first = false;
    is_capture_char(c, first);
}

#[test]
fn test_is_capture_char_with_valid_char_close_bracket() {
    let c = ']';
    let first = false;
    is_capture_char(c, first);
}

#[test]
fn test_is_capture_char_with_valid_char_alphanumeric_a() {
    let c = 'a';
    let first = false;
    is_capture_char(c, first);
}

#[test]
fn test_is_capture_char_with_valid_char_alphanumeric_1() {
    let c = '1';
    let first = false;
    is_capture_char(c, first);
}

#[test]
fn test_is_capture_char_with_valid_char_alphanumeric_z() {
    let c = 'z';
    let first = false;
    is_capture_char(c, first);
}

#[test]
fn test_is_capture_char_with_valid_char_alphanumeric_A() {
    let c = 'A';
    let first = false;
    is_capture_char(c, first);
}

#[test]
fn test_is_capture_char_with_valid_char_alphanumeric_9() {
    let c = '9';
    let first = false;
    is_capture_char(c, first);
}

