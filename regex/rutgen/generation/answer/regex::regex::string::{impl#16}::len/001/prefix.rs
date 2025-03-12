// Answer 0

#[test]
fn test_len_empty_regex() {
    let re = regex::Regex::new("").unwrap();
    let locs = re.capture_locations();
    locs.len();  // Should return 1
}

#[test]
fn test_len_no_matching_regex() {
    let re = regex::Regex::new("[a&&b]").unwrap();
    let locs = re.capture_locations();
    locs.len();  // Should return 1
}

#[test]
fn test_len_simple_regex() {
    let re = regex::Regex::new(r"(\w+)").unwrap();
    let mut locs = re.capture_locations();
    locs.len();  // Should return 2
    re.captures_read(&mut locs, "Hello").unwrap();
    locs.len();  // Should still return 2
}

#[test]
fn test_len_named_capturing_groups() {
    let re = regex::Regex::new(r"(?<first>\w+)\s+(?<last>\w+)").unwrap();
    let mut locs = re.capture_locations();
    locs.len();  // Should return 3
    re.captures_read(&mut locs, "Bruce Springsteen").unwrap();
    locs.len();  // Should still return 3
}

#[test]
fn test_len_complex_regex() {
    let re = regex::Regex::new(r"(?P<word1>\w+)(?P<word2>\w+)").unwrap();
    let mut locs = re.capture_locations();
    locs.len();  // Should return 3
    re.captures_read(&mut locs, "foo bar").unwrap();
    locs.len();  // Should still return 3
}

#[test]
fn test_len_multiple_matches() {
    let re = regex::Regex::new(r"(\d+)").unwrap();
    let mut locs = re.capture_locations();
    locs.len();  // Should return 2
    re.captures_read(&mut locs, "123").unwrap();
    locs.len();  // Should still return 2
}

