// Answer 0

#[test]
#[should_panic]
fn test_find_rev_empty_needles() {
    let needles: &[u8] = &[];
    let haystack: &[u8] = b"example data";
    let at: usize = 0;
    let _result = find_rev(needles, haystack, at);
}

#[test]
#[should_panic]
fn test_find_rev_empty_needles_at_non_zero() {
    let needles: &[u8] = &[];
    let haystack: &[u8] = b"example data";
    let at: usize = 5;
    let _result = find_rev(needles, haystack, at);
}

