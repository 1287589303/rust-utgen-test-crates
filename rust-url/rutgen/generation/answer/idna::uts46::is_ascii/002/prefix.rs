// Answer 0

#[test]
fn test_is_ascii_with_non_ascii_characters() {
    let label: &[char] = &['A', 'B', 'C', 'é'];
    let result = is_ascii(label);
}

#[test]
fn test_is_ascii_with_single_non_ascii_character() {
    let label: &[char] = &['ñ'];
    let result = is_ascii(label);
}

#[test]
fn test_is_ascii_with_multiple_non_ascii_characters() {
    let label: &[char] = &['Δ', 'Ω', '©'];
    let result = is_ascii(label);
}

#[test]
fn test_is_ascii_with_non_ascii_character_in_mixed_label() {
    let label: &[char] = &['H', 'e', 'l', 'l', 'ø', '!', ' '];
    let result = is_ascii(label);
}

#[test]
fn test_is_ascii_with_max_length_label() {
    let mut label: Vec<char> = vec!['A'; 1000]; // 1000 ASCII characters
    label.push('é'); // Add a non-ASCII character
    let result = is_ascii(&label);
}

