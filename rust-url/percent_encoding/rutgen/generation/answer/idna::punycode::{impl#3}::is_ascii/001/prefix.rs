// Answer 0

#[test]
fn test_is_ascii_lowercase_a() {
    let input: char = 'a';
    let result = input.is_ascii();
}

#[test]
fn test_is_ascii_uppercase_A() {
    let input: char = 'A';
    let result = input.is_ascii();
}

#[test]
fn test_is_ascii_digit_0() {
    let input: char = '0';
    let result = input.is_ascii();
}

#[test]
fn test_is_ascii_symbol_exclamation() {
    let input: char = '!';
    let result = input.is_ascii();
}

#[test]
fn test_is_ascii_boundary_delimiter() {
    let input: char = '\x7F'; // Boundary of ASCII values
    let result = input.is_ascii();
}

#[test]
fn test_is_ascii_non_ascii_char() {
    let input: char = 'é';
    let result = input.is_ascii();
}

#[test]
fn test_is_ascii_unicode_char() {
    let input: char = '😊'; // Emoji
    let result = input.is_ascii();
}

