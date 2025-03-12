// Answer 0

#[test]
fn test_is_ascii_empty_slice() {
    let label: &[char] = &[];
    is_ascii(label);
}

#[test]
fn test_is_ascii_single_ascii_character() {
    let label: &[char] = &['a'];
    is_ascii(label);
}

#[test]
fn test_is_ascii_multiple_ascii_characters() {
    let label: &[char] = &['h', 'e', 'l', 'l', 'o'];
    is_ascii(label);
}

#[test]
fn test_is_ascii_boundary_case_maximum_length() {
    let label: Vec<char> = (0..1000).map(|i| char::from(('a' as u8 + (i % 26)))).collect();
    is_ascii(&label);
}

#[test]
fn test_is_ascii_with_empty_slice() {
    let label: &[char] = &[];
    is_ascii(label);
}

#[test]
fn test_is_ascii_various_ascii_characters() {
    let label: &[char] = &['A', 'B', 'C', '1', '2', '3', '!', '@', '#', '$'];
    is_ascii(label);
}

#[test]
fn test_is_ascii_large_input() {
    let label: Vec<char> = (0..2000).map(|i| char::from(('a' as u8 + (i % 26)))).collect();
    is_ascii(&label);
}

