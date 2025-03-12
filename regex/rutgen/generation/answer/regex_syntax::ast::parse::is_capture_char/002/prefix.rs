// Answer 0

#[test]
fn test_is_capture_char_first_true_underscore() {
    let c = '_';
    let first = true;
    let result = is_capture_char(c, first);
}

#[test]
fn test_is_capture_char_first_true_a() {
    let c = 'a';
    let first = true;
    let result = is_capture_char(c, first);
}

#[test]
fn test_is_capture_char_first_true_A() {
    let c = 'A';
    let first = true;
    let result = is_capture_char(c, first);
}

#[test]
fn test_is_capture_char_first_true_z() {
    let c = 'z';
    let first = true;
    let result = is_capture_char(c, first);
}

#[test]
fn test_is_capture_char_first_true_Z() {
    let c = 'Z';
    let first = true;
    let result = is_capture_char(c, first);
}

#[test]
fn test_is_capture_char_first_true_invalid() {
    let c = '1';
    let first = true;
    let result = is_capture_char(c, first);
}

