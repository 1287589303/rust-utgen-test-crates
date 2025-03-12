// Answer 0

#[test]
fn test_set_lower_valid_character() {
    let mut range = ClassUnicodeRange::default();
    range.set_lower('A');
}

#[test]
fn test_set_lower_valid_character_boundary_lower() {
    let mut range = ClassUnicodeRange::default();
    range.set_lower('\u{0000}');
}

#[test]
fn test_set_lower_valid_character_boundary_upper() {
    let mut range = ClassUnicodeRange::default();
    range.set_lower('\u{10FFFF}');
}

