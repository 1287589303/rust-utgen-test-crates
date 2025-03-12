// Answer 0

#[test]
fn test_symbolic_name_normalize_bytes_empty_slice() {
    let mut slice: &mut [u8] = &mut [];
    symbolic_name_normalize_bytes(&mut slice);
}

#[test]
fn test_symbolic_name_normalize_bytes_single_non_ascii() {
    let mut slice: &mut [u8] = &mut [0x80];
    symbolic_name_normalize_bytes(&mut slice);
}

#[test]
fn test_symbolic_name_normalize_bytes_two_non_ascii() {
    let mut slice: &mut [u8] = &mut [0x80, 0x81];
    symbolic_name_normalize_bytes(&mut slice);
}

#[test]
fn test_symbolic_name_normalize_bytes_mixed_non_ascii_and_ascii() {
    let mut slice: &mut [u8] = &mut [0x80, b'a'];
    symbolic_name_normalize_bytes(&mut slice);
}

