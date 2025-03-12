// Answer 0

#[test]
fn test_is_capture_char_invalid_character1() {
    let c = '#'; // not '_', not alphanumeric, and not '.' or '[' or ']'
    let first = false;
    is_capture_char(c, first);
}

#[test]
fn test_is_capture_char_invalid_character2() {
    let c = '@'; // not '_', not alphanumeric, and not '.' or '[' or ']'
    let first = false;
    is_capture_char(c, first);
}

#[test]
fn test_is_capture_char_invalid_character3() {
    let c = '%'; // not '_', not alphanumeric, and not '.' or '[' or ']'
    let first = false;
    is_capture_char(c, first);
}

#[test]
fn test_is_capture_char_invalid_character4() {
    let c = ' '; // whitespace; not '_', not alphanumeric, and not '.' or '[' or ']'
    let first = false;
    is_capture_char(c, first);
}

#[test]
fn test_is_capture_char_invalid_character5() {
    let c = '`'; // not '_', not alphanumeric, and not '.' or '[' or ']'
    let first = false;
    is_capture_char(c, first);
}

