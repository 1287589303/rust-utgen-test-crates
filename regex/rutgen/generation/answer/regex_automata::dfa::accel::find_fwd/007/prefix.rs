// Answer 0

#[test]
fn test_find_fwd_single_needle_none_found_at_start() {
    let needles: &[u8] = &[1u8];
    let haystack: &[u8] = &[0u8; 10];
    let at: usize = 0;
    let _result = find_fwd(needles, haystack, at);
}

#[test]
fn test_find_fwd_single_needle_none_found_at_mid() {
    let needles: &[u8] = &[1u8];
    let haystack: &[u8] = &[0u8; 10];
    let at: usize = 5;
    let _result = find_fwd(needles, haystack, at);
}

#[test]
fn test_find_fwd_single_needle_none_found_at_end() {
    let needles: &[u8] = &[1u8];
    let haystack: &[u8] = &[0u8; 10];
    let at: usize = 9;
    let _result = find_fwd(needles, haystack, at);
}

#[test]
fn test_find_fwd_single_needle_none_found_at_out_of_bounds() {
    let needles: &[u8] = &[1u8];
    let haystack: &[u8] = &[0u8; 10];
    let at: usize = 10; // out of bounds
    let _result = find_fwd(needles, haystack, at);
}

