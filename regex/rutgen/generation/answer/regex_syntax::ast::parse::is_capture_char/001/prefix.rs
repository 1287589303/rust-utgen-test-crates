// Answer 0

#[test]
fn test_is_capture_char_first_true_non_alpha_non_underscore() {
    let test_cases = ['1', '2', '3', '!', '@', '#', '%', '^', '&', '*', '(', ')', '-', '=', '+', '\0'];
    for &c in &test_cases {
        let result = is_capture_char(c, true);
    }
}

#[test]
fn test_is_capture_char_first_true_special_characters() {
    let special_characters = ['$', '>', '<', '|', '/', '\\', ':', ';', '\"', '\''];
    for &c in &special_characters {
        let result = is_capture_char(c, true);
    }
}

#[test]
fn test_is_capture_char_first_true_uppercase_non_alpha() {
    let uppercase_non_alpha = ['@', '#', '$', '%', '^', '&', '*'];
    for &c in &uppercase_non_alpha {
        let result = is_capture_char(c, true);
    }
}

#[test]
fn test_is_capture_char_first_true_lowercase_non_alpha() {
    let lowercase_non_alpha = ['`', '~', '-', '=', '+', ',', '.', '<', '>'];
    for &c in &lowercase_non_alpha {
        let result = is_capture_char(c, true);
    }
}

