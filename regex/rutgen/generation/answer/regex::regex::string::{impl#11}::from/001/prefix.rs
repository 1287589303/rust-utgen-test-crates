// Answer 0

#[test]
fn test_match_range_valid_bounds() {
    let haystack = "example";
    for start in 0..=haystack.len() {
        for end in start..=haystack.len() {
            let m = Match::new(haystack, start, end);
            let range: core::ops::Range<usize> = m.into();
        }
    }
}

#[test]
fn test_match_range_empty_match() {
    let haystack = "example";
    let m = Match::new(haystack, 0, 0);
    let range: core::ops::Range<usize> = m.into();
}

#[test]
fn test_match_range_full_match() {
    let haystack = "example";
    let m = Match::new(haystack, 0, haystack.len());
    let range: core::ops::Range<usize> = m.into();
}

#[test]
fn test_match_range_single_character_match() {
    let haystack = "example";
    let m = Match::new(haystack, 2, 3);
    let range: core::ops::Range<usize> = m.into();
}

#[test]
#[should_panic]
fn test_match_range_invalid_bounds() {
    let haystack = "example";
    let _ = Match::new(haystack, 5, 2); // This should cause a panic due to invalid range
}

