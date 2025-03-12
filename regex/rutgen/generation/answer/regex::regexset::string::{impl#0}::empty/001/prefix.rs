// Answer 0

#[test]
fn test_empty_regex_set() {
    let set = RegexSet::empty();
    let _ = set.is_empty();
    let _ = set.is_match("");
}

#[test]
fn test_is_match_at_out_of_bounds() {
    let set = RegexSet::empty();
    let haystack = "test string";
    
    let _ = set.is_match_at(haystack, usize::MAX); // out of bounds
    let _ = set.is_match_at(haystack, 12); // out of bounds (greater than string length)
    let _ = set.is_match_at(haystack, 0); // within bounds (should always return false for empty set)
}

