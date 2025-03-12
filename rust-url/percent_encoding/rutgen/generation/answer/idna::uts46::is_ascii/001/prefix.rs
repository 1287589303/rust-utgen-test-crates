// Answer 0

#[test]
fn test_is_ascii_empty() {
    let label: &[char] = &[];
    is_ascii(label);
}

#[test]
fn test_is_ascii_only_ascii() {
    let label: &[char] = &['a', 'b', 'c', '1', '2', '3', '!', '@', '#', '$'];
    is_ascii(label);
}

#[test]
fn test_is_ascii_max_length() {
    let label: &[char] = &['a'; 2000]; // 2000 ASCII characters
    is_ascii(label);
}

#[test]
fn test_is_ascii_with_non_ascii() {
    let label: &[char] = &['a', 'b', 'c', 'á']; // Contains a non-ASCII character
    is_ascii(label);
}

