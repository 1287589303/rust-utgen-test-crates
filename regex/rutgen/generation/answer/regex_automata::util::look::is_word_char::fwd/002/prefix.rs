// Answer 0

#[test]
fn test_fwd_valid_utf8() {
    let haystack: &[u8] = b"valid";
    let at: usize = 0;
    let _ = fwd(haystack, at);
}

#[test]
fn test_fwd_invalid_utf8_sequence() {
    let haystack: &[u8] = &[0xFF]; // Invalid UTF-8 byte
    let at: usize = 0;
    let _ = fwd(haystack, at);
}

#[test]
fn test_fwd_empty_byte_slice() {
    let haystack: &[u8] = &[];
    let at: usize = 0; // At the start of an empty slice
    let _ = fwd(haystack, at);
}

#[test]
fn test_fwd_at_middle_of_valid_utf8() {
    let haystack: &[u8] = b"example";
    let at: usize = 3; // Starting at 'm'
    let _ = fwd(haystack, at);
}

#[test]
fn test_fwd_at_end_of_valid_utf8() {
    let haystack: &[u8] = b"end";
    let at: usize = 2; // At the end of the slice
    let _ = fwd(haystack, at);
}

#[test]
fn test_fwd_partial_invalid_utf8() {
    let haystack: &[u8] = b"partial \xFF"; // Valid followed by invalid
    let at: usize = 8; // At the position of the invalid byte
    let _ = fwd(haystack, at);
}

