// Answer 0

#[test]
fn test_digit_non_ascii_character() {
    let input: char = 'ñ'; // Non-ASCII character
    let result = input.digit();
}

#[test]
fn test_digit_special_character() {
    let input: char = '@'; // Special character
    let result = input.digit();
}

#[test]
fn test_digit_uppercase_letter() {
    let input: char = 'G'; // Uppercase letter
    let result = input.digit();
}

#[test]
fn test_digit_space_character() {
    let input: char = ' '; // Space character
    let result = input.digit();
}

#[test]
fn test_digit_control_character() {
    let input: char = '\n'; // Control character
    let result = input.digit();
}

