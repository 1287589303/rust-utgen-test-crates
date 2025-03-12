// Answer 0

#[test]
fn test_pattern_valid_case_zero() {
    let pattern_id = PatternID(0);
    let half_match = HalfMatch::new(pattern_id, 0);
    half_match.pattern();
}

#[test]
fn test_pattern_valid_case_maximum() {
    let pattern_id = PatternID(u16::MAX);
    let half_match = HalfMatch::new(pattern_id, 0);
    half_match.pattern();
}

#[test]
fn test_pattern_valid_case_mid_range() {
    let pattern_id = PatternID(u16::MAX / 2);
    let half_match = HalfMatch::new(pattern_id, 0);
    half_match.pattern();
}

#[test]
fn test_pattern_valid_case_non_zero_offset() {
    let pattern_id = PatternID(1);
    let half_match = HalfMatch::new(pattern_id, 10);
    half_match.pattern();
}

#[test]
fn test_pattern_valid_case_large_offset() {
    let pattern_id = PatternID(2);
    let half_match = HalfMatch::new(pattern_id, usize::MAX);
    half_match.pattern();
}

