// Answer 0

#[test]
fn test_find_rev_three_needles() {
    let needles = &[1, 2, 3];
    let haystack = &[0, 1, 2, 3, 2, 1, 0];
    let at = 7;
    let _ = find_rev(needles, haystack, at);
}

#[test]
fn test_find_rev_one_needle() {
    let needles = &[1];
    let haystack = &[0, 1, 2, 3, 1, 0];
    let at = 5;
    let _ = find_rev(needles, haystack, at);
}

#[test]
fn test_find_rev_two_needles() {
    let needles = &[1, 2];
    let haystack = &[0, 2, 1, 2, 3, 0];
    let at = 6;
    let _ = find_rev(needles, haystack, at);
}

#[should_panic]
#[test]
fn test_find_rev_empty_needles() {
    let needles: &[u8] = &[];
    let haystack = &[0, 1, 2, 3];
    let at = 4;
    let _ = find_rev(needles, haystack, at);
}

