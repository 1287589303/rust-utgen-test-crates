// Answer 0

#[test]
fn test_matches_at_with_zero_start() {
    let set = RegexSet::new([r"\bfoo\b", r"bar"]).unwrap();
    let haystack = "foobar";
    let result = set.matches_at(haystack, 0);
}

#[test]
fn test_matches_at_with_middle_start() {
    let set = RegexSet::new([r"\bfoo\b", r"bar"]).unwrap();
    let haystack = "foobar";
    let result = set.matches_at(haystack, 3);
}

#[test]
fn test_matches_at_with_end_start() {
    let set = RegexSet::new([r"\bfoo\b", r"bar"]).unwrap();
    let haystack = "foobar";
    let result = set.matches_at(haystack, 6);
}

#[test]
fn test_matches_at_with_empty_string() {
    let set = RegexSet::new([r"\bfoo\b", r"bar"]).unwrap();
    let haystack = "";
    let result = set.matches_at(haystack, 0);
}

#[test]
fn test_matches_at_with_single_character_matching() {
    let set = RegexSet::new([r"a"]).unwrap();
    let haystack = "a";
    let result = set.matches_at(haystack, 0);
}

#[test]
#[should_panic]
fn test_matches_at_with_out_of_bounds_start() {
    let set = RegexSet::new([r"\bfoo\b", r"bar"]).unwrap();
    let haystack = "foobar";
    let result = set.matches_at(haystack, 7);
}

#[test]
fn test_matches_at_with_multiple_patterns() {
    let set = RegexSet::new([r"foo", r"bar", r"baz"]).unwrap();
    let haystack = "foobarbaz";
    let result = set.matches_at(haystack, 3);
}

