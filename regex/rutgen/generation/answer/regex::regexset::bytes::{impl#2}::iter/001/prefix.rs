// Answer 0

#[test]
fn test_iter_with_matching_input() {
    let set = SetMatches(PatternSet::new(vec![
        PatternID::from_regex(r"[0-9]").unwrap(),
        PatternID::from_regex(r"[A-Za-z]").unwrap(),
        PatternID::from_regex(r"\p{Greek}").unwrap(),
    ]));
    let hay = "abc123".as_bytes();
    let _matches_iter = set.iter();
}

#[test]
fn test_iter_with_partial_matching_input() {
    let set = SetMatches(PatternSet::new(vec![
        PatternID::from_regex(r"[0-9]").unwrap(),
        PatternID::from_regex(r"[A-Za-z]").unwrap(),
        PatternID::from_regex(r"\p{Greek}").unwrap(),
    ]));
    let hay = "β".as_bytes();
    let _matches_iter = set.iter();
}

#[test]
fn test_iter_with_no_matching_input() {
    let set = SetMatches(PatternSet::new(vec![
        PatternID::from_regex(r"[0-9]").unwrap(),
        PatternID::from_regex(r"[A-Za-z]").unwrap(),
        PatternID::from_regex(r"\p{Greek}").unwrap(),
    ]));
    let hay = "!!!".as_bytes();
    let _matches_iter = set.iter();
}

#[test]
fn test_iter_with_empty_input() {
    let set = SetMatches(PatternSet::new(vec![
        PatternID::from_regex(r"[0-9]").unwrap(),
        PatternID::from_regex(r"[A-Za-z]").unwrap(),
        PatternID::from_regex(r"\p{Greek}").unwrap(),
    ]));
    let hay = "".as_bytes();
    let _matches_iter = set.iter();
}

#[test]
fn test_iter_with_non_matching_edge_case() {
    let set = SetMatches(PatternSet::new(vec![
        PatternID::from_regex(r"[0-9]").unwrap(),
        PatternID::from_regex(r"[A-Za-z]").unwrap(),
        PatternID::from_regex(r"\p{Greek}").unwrap(),
    ]));
    let hay = "123abcβ".as_bytes();
    let _matches_iter = set.iter();
}

