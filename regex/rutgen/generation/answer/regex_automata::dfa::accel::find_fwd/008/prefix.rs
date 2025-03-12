// Answer 0

#[test]
fn test_find_fwd_single_needle_found() {
    let needles: &[u8] = &[b'a'];
    let haystack: &[u8] = b"abcde";
    let at: usize = 0;
    let result = find_fwd(needles, haystack, at);
}

#[test]
fn test_find_fwd_single_needle_found_at_non_zero() {
    let needles: &[u8] = &[b'b'];
    let haystack: &[u8] = b"abcde";
    let at: usize = 1;
    let result = find_fwd(needles, haystack, at);
}

#[test]
fn test_find_fwd_single_needle_found_with_multiple_occurrences() {
    let needles: &[u8] = &[b'e'];
    let haystack: &[u8] = b"abcdeee";
    let at: usize = 3;
    let result = find_fwd(needles, haystack, at);
}

#[test]
fn test_find_fwd_single_needle_found_at_end() {
    let needles: &[u8] = &[b'e'];
    let haystack: &[u8] = b"abcde";
    let at: usize = 4;
    let result = find_fwd(needles, haystack, at);
}

