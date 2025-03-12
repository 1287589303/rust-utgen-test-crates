// Answer 0

#[test]
fn test_is_ascii_with_ascii_character() {
    let c: char = 'A';
    let result = c.is_ascii();
}

#[test]
fn test_is_ascii_with_non_ascii_character() {
    let c: char = 'é';
    let result = c.is_ascii();
}

#[test]
fn test_is_ascii_with_lowercase_ascii_character() {
    let c: char = 'a';
    let result = c.is_ascii();
}

#[test]
fn test_is_ascii_with_numeric_ascii_character() {
    let c: char = '3';
    let result = c.is_ascii();
}

#[test]
fn test_is_ascii_with_special_character() {
    let c: char = '#';
    let result = c.is_ascii();
}

#[test]
fn test_is_ascii_with_boundary_character() {
    let c: char = '\x7F'; // DEL character
    let result = c.is_ascii();
}

