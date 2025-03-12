// Answer 0

#[test]
fn test_find_rev_with_three_needles() {
    let needles: [u8; 3] = [1, 2, 3];
    let haystack: [u8; 5] = [0, 1, 2, 3, 4];
    let at: usize = 5;
    let _ = find_rev(&needles, &haystack, at);
}

#[test]
fn test_find_rev_with_three_needles_at_boundary() {
    let needles: [u8; 3] = [1, 2, 3];
    let haystack: [u8; 4] = [2, 1, 3, 4];
    let at: usize = 4;
    let _ = find_rev(&needles, &haystack, at);
}

#[test]
fn test_find_rev_with_three_needles_no_match() {
    let needles: [u8; 3] = [5, 6, 7];
    let haystack: [u8; 6] = [0, 1, 2, 3, 4, 8];
    let at: usize = 6;
    let _ = find_rev(&needles, &haystack, at);
}

