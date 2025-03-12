// Answer 0

#[test]
fn test_is_capture_char_first_false() {
    let test_inputs = vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '!', '@', '#', '$', '%', '^', '&', '*', '(', ')', '-', '=', '+', '{', '}', '|', ':', ';', '"', '\'', '<', '>', ',', '.', '?', '/', '\\'];
    for &c in &test_inputs {
        let result = is_capture_char(c, true);
    }
}

#[test]
fn test_is_capture_char_first_false_special_chars() {
    let special_chars = vec!['`', '~', ' ', '\t', '\n'];
    for &c in &special_chars {
        let result = is_capture_char(c, true);
    }
}

