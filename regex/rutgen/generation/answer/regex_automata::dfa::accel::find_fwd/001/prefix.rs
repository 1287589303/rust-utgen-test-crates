// Answer 0

#[test]
#[should_panic]
fn test_find_fwd_empty_needles() {
    let needles: &[u8] = &[];
    let haystack: &[u8] = b"example";
    let at = 0;
    let _ = find_fwd(needles, haystack, at);
}

#[test]
fn test_find_fwd_one_needle() {
    let needles: &[u8] = &[0x41]; // 'A'
    let haystack: &[u8] = b"example with A inside";
    let at = 0;
    let _ = find_fwd(needles, haystack, at);
}

#[test]
fn test_find_fwd_two_needles() {
    let needles: &[u8] = &[0x41, 0x42]; // 'A' and 'B'
    let haystack: &[u8] = b"example with A inside";
    let at = 0;
    let _ = find_fwd(needles, haystack, at);
}

#[test]
fn test_find_fwd_three_needles() {
    let needles: &[u8] = &[0x41, 0x42, 0x43]; // 'A', 'B', and 'C'
    let haystack: &[u8] = b"example with A, B, and C inside";
    let at = 0;
    let _ = find_fwd(needles, haystack, at);
}

