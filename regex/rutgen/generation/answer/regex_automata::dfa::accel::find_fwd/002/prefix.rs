// Answer 0

#[test]
#[should_panic]
fn test_find_fwd_with_empty_needles() {
    let needles: &[u8] = &[];
    let haystack: &[u8] = b"test haystack";
    let at: usize = 0;
    find_fwd(needles, haystack, at);
}

