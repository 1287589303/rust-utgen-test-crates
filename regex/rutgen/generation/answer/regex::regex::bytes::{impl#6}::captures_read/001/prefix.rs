// Answer 0

#[test]
fn test_captures_read_basic_match() {
    let re = Regex::new(r"^([a-z]+)=(\S*)$").unwrap();
    let mut locs = re.capture_locations();
    let haystack = b"id=foo123";
    let result = re.captures_read(&mut locs, haystack);
}

#[test]
fn test_captures_read_exact_match() {
    let re = Regex::new(r"^(test)=(value)$").unwrap();
    let mut locs = re.capture_locations();
    let haystack = b"test=value";
    let result = re.captures_read(&mut locs, haystack);
}

#[test]
fn test_captures_read_no_match() {
    let re = Regex::new(r"^([a-z]+)=(\S*)$").unwrap();
    let mut locs = re.capture_locations();
    let haystack = b"invalid_format";
    let result = re.captures_read(&mut locs, haystack);
}

#[test]
fn test_captures_read_empty_haystack() {
    let re = Regex::new(r"^([a-z]+)=(\S*)$").unwrap();
    let mut locs = re.capture_locations();
    let haystack: &[u8] = b"";
    let result = re.captures_read(&mut locs, haystack);
}

#[test]
fn test_captures_read_multiple_matches() {
    let re = Regex::new(r"^([a-z]+)=(\S*)$").unwrap();
    let mut locs = re.capture_locations();
    let haystack = b"first=value1\nsecond=value2";
    let result1 = re.captures_read(&mut locs, &haystack[0..14]);
    let result2 = re.captures_read(&mut locs, &haystack[15..30]);
}

