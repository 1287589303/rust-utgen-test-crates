// Answer 0

#[test]
fn test_matched_any_empty_pattern_set() {
    let empty_pattern_set = PatternSet::new(vec![]).unwrap();
    let set_matches = SetMatches(empty_pattern_set);
    let _ = set_matches.matched_any();
}

#[test]
fn test_matched_any_single_pattern_match() {
    let pattern_set = PatternSet::new(vec![
        r"[a-z]+@[a-z]+\.(com|org|net)",
    ]).unwrap();
    let set_matches = SetMatches(pattern_set);
    let _ = set_matches.matched_any();
}

#[test]
fn test_matched_any_multiple_patterns_with_match() {
    let pattern_set = PatternSet::new(vec![
        r"[a-z]+@[a-z]+\.(com|org|net)",
        r"[A-Z]+\s+\d{3}",
    ]).unwrap();
    let set_matches = SetMatches(pattern_set);
    let _ = set_matches.matched_any();
}

#[test]
fn test_matched_any_multiple_patterns_without_match() {
    let pattern_set = PatternSet::new(vec![
        r"\d{3}-\d{2}-\d{4}",
        r"[a-z]+\.(com|net)",
    ]).unwrap();
    let set_matches = SetMatches(pattern_set);
    let _ = set_matches.matched_any();
}

