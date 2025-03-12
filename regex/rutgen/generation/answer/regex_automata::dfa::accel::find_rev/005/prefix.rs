// Answer 0

#[test]
fn test_find_rev_single_needle_found() {
    let needles: &[u8] = &[42]; // Example needle
    let haystack: &[u8] = &[1, 2, 42, 3, 4]; // Needle is present
    let at = haystack.len();
    let _ = find_rev(needles, haystack, at);
}

#[test]
fn test_find_rev_single_needle_not_found() {
    let needles: &[u8] = &[255]; // Example needle not in haystack
    let haystack: &[u8] = &[1, 2, 42, 3, 4];
    let at = haystack.len();
    let _ = find_rev(needles, haystack, at);
}

#[test]
fn test_find_rev_single_needle_edge_case_at_zero() {
    let needles: &[u8] = &[2]; // Needle at first position
    let haystack: &[u8] = &[2, 3, 4, 5];
    let at = 1; // Only looking at first element
    let _ = find_rev(needles, haystack, at);
}

#[test]
fn test_find_rev_single_needle_empty_haystack() {
    let needles: &[u8] = &[1]; // Any single needle
    let haystack: &[u8] = &[];
    let at = 0; // the haystack is empty
    let _ = find_rev(needles, haystack, at);
}

