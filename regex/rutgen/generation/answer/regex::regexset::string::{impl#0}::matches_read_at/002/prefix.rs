// Answer 0

#[test]
fn test_matches_read_at_no_match_start_zero() {
    let regex_set = RegexSet::new(vec!["a.*b"]).unwrap();
    let mut matches = vec![false; regex_set.len()];
    let haystack = "cd";
    let start = 0;
    regex_set.matches_read_at(&mut matches, haystack, start);
}

#[test]
fn test_matches_read_at_no_match_start_one() {
    let regex_set = RegexSet::new(vec!["a.*b"]).unwrap();
    let mut matches = vec![false; regex_set.len()];
    let haystack = "cde";
    let start = 1;
    regex_set.matches_read_at(&mut matches, haystack, start);
}

#[test]
fn test_matches_read_at_no_match_middle() {
    let regex_set = RegexSet::new(vec!["x.*y"]).unwrap();
    let mut matches = vec![false; regex_set.len()];
    let haystack = "abcde";
    let start = 2;
    regex_set.matches_read_at(&mut matches, haystack, start);
}

#[test]
fn test_matches_read_at_no_match_at_end() {
    let regex_set = RegexSet::new(vec!["c.*d"]).unwrap();
    let mut matches = vec![false; regex_set.len()];
    let haystack = "abc";
    let start = 2;
    regex_set.matches_read_at(&mut matches, haystack, start);
}

