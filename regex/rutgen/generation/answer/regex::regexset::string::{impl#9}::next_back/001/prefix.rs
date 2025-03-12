// Answer 0

#[test]
fn test_next_back_non_empty_iterator() {
    let patterns = vec!["abc", "def", "ghi"];
    let regex_set_builder = RegexSetBuilder::new();
    let pattern_set = regex_set_builder.add_patterns(&patterns).build().unwrap();
    let pattern_set_iter = pattern_set.iter();
    let mut set_matches_iter = SetMatchesIter(pattern_set_iter);

    let _ = set_matches_iter.next_back(); // Should return Some(index) for last pattern
    let _ = set_matches_iter.next_back(); // Should return Some(index) for second pattern
    let _ = set_matches_iter.next_back(); // Should return Some(index) for first pattern
    let _ = set_matches_iter.next_back(); // Should return None for exhausted iterator
}

#[test]
fn test_next_back_exhausted_iterator() {
    let patterns = vec!["abc"];
    let regex_set_builder = RegexSetBuilder::new();
    let pattern_set = regex_set_builder.add_patterns(&patterns).build().unwrap();
    let pattern_set_iter = pattern_set.iter();
    let mut set_matches_iter = SetMatchesIter(pattern_set_iter);

    let _ = set_matches_iter.next_back(); // Should return Some(0) for first pattern
    let _ = set_matches_iter.next_back(); // Should return None for exhausted iterator
}

#[test]
fn test_next_back_with_edge_case_ids() {
    let patterns = vec!["abc", "def", "ghi", "jkl", "mno"];
    let regex_set_builder = RegexSetBuilder::new();
    let pattern_set = regex_set_builder.add_patterns(&patterns).build().unwrap();
    let pattern_set_iter = pattern_set.iter();
    let mut set_matches_iter = SetMatchesIter(pattern_set_iter);

    let _ = set_matches_iter.next_back(); // Should return Some(4) for last pattern "mno"
    let _ = set_matches_iter.next_back(); // Should return Some(3) for "jkl"
    let _ = set_matches_iter.next_back(); // Should return Some(2) for "ghi"
    let _ = set_matches_iter.next_back(); // Should return Some(1) for "def"
    let _ = set_matches_iter.next_back(); // Should return Some(0) for "abc"
    let _ = set_matches_iter.next_back(); // Should return None for exhausted iterator
}

