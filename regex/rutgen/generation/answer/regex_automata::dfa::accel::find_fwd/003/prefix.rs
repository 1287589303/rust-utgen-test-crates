// Answer 0

#[test]
fn test_find_fwd_needles_len_3_no_match() {
    let needles: &[u8] = &[1, 2, 3];
    let haystack: &[u8] = &[4, 5, 6];
    let at: usize = 0;
    let _ = find_fwd(needles, haystack, at);
}

#[test]
fn test_find_fwd_needles_len_3_no_match_at_end() {
    let needles: &[u8] = &[10, 11, 12];
    let haystack: &[u8] = &[13, 14, 15];
    let at: usize = 2;
    let _ = find_fwd(needles, haystack, at);
}

#[test]
fn test_find_fwd_needles_len_3_no_match_middle_index() {
    let needles: &[u8] = &[20, 21, 22];
    let haystack: &[u8] = &[23, 24, 25];
    let at: usize = 1;
    let _ = find_fwd(needles, haystack, at);
}

