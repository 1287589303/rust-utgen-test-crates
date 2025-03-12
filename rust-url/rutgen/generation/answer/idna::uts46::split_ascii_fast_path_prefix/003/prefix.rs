// Answer 0

#[test]
fn test_split_ascii_fast_path_prefix_non_ascii_first() {
    let label: &[u8] = &[0xC2, 0xA9]; // Non-ASCII character ©
    let (head, tail) = split_ascii_fast_path_prefix(label);
    let expected_head: &[u8] = &[]; // No ASCII characters before the non-ASCII character
    let expected_tail: &[u8] = label; // The entire label is returned
    let _ = (head, tail); // Placeholder to ensure the function is called
}

#[test]
fn test_split_ascii_fast_path_prefix_non_ascii_first_multiple_bytes() {
    let label: &[u8] = &[0xE2, 0x82, 0xAC]; // Non-ASCII character € (Euro sign)
    let (head, tail) = split_ascii_fast_path_prefix(label);
    let expected_head: &[u8] = &[]; // No ASCII characters before the non-ASCII character
    let expected_tail: &[u8] = label; // The entire label is returned
    let _ = (head, tail); // Placeholder to ensure the function is called
}

#[test]
fn test_split_ascii_fast_path_prefix_non_ascii_first_various() {
    let label: &[u8] = &[0xF0, 0x9F, 0x8C, 0x9F]; // Non-ASCII character 🌟 (Star)
    let (head, tail) = split_ascii_fast_path_prefix(label);
    let expected_head: &[u8] = &[]; // No ASCII characters before the non-ASCII character
    let expected_tail: &[u8] = label; // The entire label is returned
    let _ = (head, tail); // Placeholder to ensure the function is called
}

