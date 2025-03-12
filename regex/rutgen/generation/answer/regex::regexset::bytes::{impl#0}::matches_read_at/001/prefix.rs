// Answer 0

#[test]
fn test_matches_read_at_with_matching_pattern() {
    let pattern = String::from("foo");
    let regex_set = RegexSet::new(vec![pattern.clone()]).unwrap();
    let mut matches = vec![false; regex_set.len()];
    let haystack = b"foobar";
    let start = 0;

    regex_set.matches_read_at(&mut matches, haystack, start);
}

#[test]
fn test_matches_read_at_with_non_matching_pattern() {
    let pattern = String::from("baz");
    let regex_set = RegexSet::new(vec![pattern.clone()]).unwrap();
    let mut matches = vec![false; regex_set.len()];
    let haystack = b"foobar";
    let start = 0;

    regex_set.matches_read_at(&mut matches, haystack, start);
}

#[test]
fn test_matches_read_at_with_empty_haystack() {
    let pattern = String::from("foo");
    let regex_set = RegexSet::new(vec![pattern.clone()]).unwrap();
    let mut matches = vec![false; regex_set.len()];
    let haystack = b"";
    let start = 0;

    regex_set.matches_read_at(&mut matches, haystack, start);
}

#[test]
fn test_matches_read_at_with_start_out_of_bounds() {
    let pattern = String::from("foo");
    let regex_set = RegexSet::new(vec![pattern.clone()]).unwrap();
    let mut matches = vec![false; regex_set.len()];
    let haystack = b"foobar";
    let start = 7; // out of bounds

    regex_set.matches_read_at(&mut matches, haystack, start);
}

#[test]
fn test_matches_read_at_on_non_matching_pattern_at_start() {
    let pattern = String::from("^bar");
    let regex_set = RegexSet::new(vec![pattern.clone()]).unwrap();
    let mut matches = vec![false; regex_set.len()];
    let haystack = b"foobar";
    let start = 0;

    regex_set.matches_read_at(&mut matches, haystack, start);
}

#[test]
fn test_matches_read_at_on_boundary_conditions() {
    let pattern = String::from("o");
    let regex_set = RegexSet::new(vec![pattern.clone()]).unwrap();
    let mut matches = vec![false; regex_set.len()];
    let haystack = b"hello world";
    let start = 4; // should match "o" in "hello"

    regex_set.matches_read_at(&mut matches, haystack, start);
}

