// Answer 0

#[test]
fn test_half_match_must_zero_pattern_and_zero_offset() {
    let half_match = HalfMatch::must(0, 0);
}

#[test]
fn test_half_match_must_minimum_pattern_and_large_offset() {
    let half_match = HalfMatch::must(0, usize::MAX);
}

#[test]
fn test_half_match_must_large_pattern_and_minimum_offset() {
    let half_match = HalfMatch::must(usize::MAX, 0);
}

#[test]
#[should_panic]
fn test_half_match_must_beyond_max_pattern() {
    let half_match = HalfMatch::must(usize::MAX + 1, 0);
}

#[test]
fn test_half_match_must_mid_pattern_and_mid_offset() {
    let half_match = HalfMatch::must(5000, 5000);
}

#[test]
fn test_half_match_must_large_pattern_and_large_offset() {
    let half_match = HalfMatch::must(10000, usize::MAX);
}

#[test]
fn test_half_match_must_pattern_and_zero_offset() {
    let half_match = HalfMatch::must(10, 0);
}

#[test]
fn test_half_match_must_pattern_zero_large_offset() {
    let half_match = HalfMatch::must(10, usize::MAX);
}

