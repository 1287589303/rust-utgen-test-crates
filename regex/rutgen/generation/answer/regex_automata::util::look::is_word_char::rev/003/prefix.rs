// Answer 0

#[test]
fn test_rev_empty_haystack() {
    let haystack: &[u8] = b"";
    let at: usize = 0;
    let _ = super::rev(haystack, at);
}

#[test]
fn test_rev_non_leading_byte_at_start() {
    let haystack: &[u8] = b"\x80"; // non-leading byte
    let at: usize = 1;
    let _ = super::rev(haystack, at);
}

#[test]
fn test_rev_non_leading_byte_in_non_empty_haystack() {
    let haystack: &[u8] = b"abc\x80"; // last byte is a non-leading byte
    let at: usize = 4; // length of haystack
    let _ = super::rev(haystack, at);
}

#[test]
fn test_rev_short_haystack_with_non_leading_byte() {
    let haystack: &[u8] = b"\x80\x80"; // two non-leading bytes
    let at: usize = 2; // length of haystack
    let _ = super::rev(haystack, at);
}

#[test]
fn test_rev_bounded_haystack_with_non_leading_byte() {
    let haystack: &[u8] = b"abc\x80"; // last byte is a non-leading byte
    let at: usize = 3;  // third byte, valid byte but not leading
    let _ = super::rev(haystack, at);
}

