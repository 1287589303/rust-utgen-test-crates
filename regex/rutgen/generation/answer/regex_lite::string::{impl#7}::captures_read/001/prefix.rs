// Answer 0

#[test]
fn test_captures_read_valid_haystack_without_capture_groups() {
    let re = Regex::new(r"abc").unwrap();
    let mut locs = re.capture_locations();
    let result = re.captures_read(&mut locs, "abc");
}

#[test]
fn test_captures_read_valid_haystack_with_multiple_capture_groups() {
    let re = Regex::new(r"^([a-z]+)=(\S*)$").unwrap();
    let mut locs = re.capture_locations();
    let result = re.captures_read(&mut locs, "id=foo123");
}

#[test]
fn test_captures_read_empty_haystack() {
    let re = Regex::new(r"abc").unwrap();
    let mut locs = re.capture_locations();
    let result = re.captures_read(&mut locs, "");
}

#[test]
fn test_captures_read_haystack_does_not_match_regex_pattern() {
    let re = Regex::new(r"abc").unwrap();
    let mut locs = re.capture_locations();
    let result = re.captures_read(&mut locs, "def");
}

#[test]
#[should_panic]
fn test_captures_read_capture_locations_not_created_for_this_regex() {
    let re1 = Regex::new(r"abc").unwrap();
    let re2 = Regex::new(r"def").unwrap();
    let mut locs = re2.capture_locations();
    let _result = re1.captures_read(&mut locs, "abc");
}

#[test]
fn test_captures_read_single_character_haystack() {
    let re = Regex::new(r"a").unwrap();
    let mut locs = re.capture_locations();
    let result = re.captures_read(&mut locs, "a");
}

#[test]
fn test_captures_read_haystack_matching_at_start_and_end() {
    let re = Regex::new(r"^abc$").unwrap();
    let mut locs = re.capture_locations();
    let result = re.captures_read(&mut locs, "abc");
}

#[test]
fn test_captures_read_large_haystack() {
    let re = Regex::new(r"a{1000}").unwrap();
    let mut locs = re.capture_locations();
    let large_haystack = "a".repeat(1000);
    let result = re.captures_read(&mut locs, &large_haystack);
}

