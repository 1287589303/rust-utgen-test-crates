// Answer 0

#[test]
fn test_matches_at_with_valid_start() {
    let set = RegexSet::new([r"foo", r"bar", r"baz"]).unwrap();
    let hay = b"foobar";
    let matches: SetMatches = set.matches_at(hay, 0);
}

#[test]
fn test_matches_at_with_boundary_start() {
    let set = RegexSet::new([r"foo"]).unwrap();
    let hay = b"foo";
    let matches: SetMatches = set.matches_at(hay, 3);
}

#[should_panic]
fn test_matches_at_with_out_of_bounds_start() {
    let set = RegexSet::new([r"foo"]).unwrap();
    let hay = b"foo";
    let _matches: SetMatches = set.matches_at(hay, 4);
}

#[test]
fn test_matches_at_with_non_matching_start() {
    let set = RegexSet::new([r"baz"]).unwrap();
    let hay = b"foobar";
    let matches: SetMatches = set.matches_at(hay, 2);
}

#[test]
fn test_matches_at_with_empty_haystack() {
    let set = RegexSet::new([r"foo"]).unwrap();
    let hay: &[u8] = b"";
    let matches: SetMatches = set.matches_at(hay, 0);
}

#[test]
fn test_matches_at_with_full_haystack_match() {
    let set = RegexSet::new([r"foobar"]).unwrap();
    let hay = b"foobar";
    let matches: SetMatches = set.matches_at(hay, 0);
}

