// Answer 0

#[test]
fn test_is_capture_char_first_true_with_underscore() {
    let c = '_';
    let first = true;
    is_capture_char(c, first);
}

#[test]
fn test_is_capture_char_first_true_with_alphabetic_A() {
    let c = 'A';
    let first = true;
    is_capture_char(c, first);
}

#[test]
fn test_is_capture_char_first_true_with_alphabetic_z() {
    let c = 'z';
    let first = true;
    is_capture_char(c, first);
}

#[test]
fn test_is_capture_char_first_true_with_numeric_1() {
    let c = '1';
    let first = true;
    is_capture_char(c, first);
}

#[test]
fn test_is_capture_char_first_true_with_special_char() {
    let c = '!'; 
    let first = true;
    is_capture_char(c, first);
}

