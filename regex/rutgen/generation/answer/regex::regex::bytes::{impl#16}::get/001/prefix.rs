// Answer 0

#[test]
fn test_get_valid_capture_group() {
    let haystack = b"Bruce Springsteen";
    let re = regex::bytes::Regex::new(r"(?P<first>\w+)\s+(?P<last>\w+)").unwrap();
    let mut locs = re.capture_locations();
    re.captures_read(&mut locs, haystack).unwrap();
    
    let _ = locs.get(0);
    let _ = locs.get(1);
    let _ = locs.get(2);
}

#[test]
fn test_get_invalid_capture_group() {
    let haystack = b"Bruce Springsteen";
    let re = regex::bytes::Regex::new(r"(?P<first>\w+)\s+(?P<last>\w+)").unwrap();
    let mut locs = re.capture_locations();
    re.captures_read(&mut locs, haystack).unwrap();

    let _ = locs.get(3);
    let _ = locs.get(4);
}

#[test]
fn test_get_capture_group_empty_input() {
    let haystack = b"";
    let re = regex::bytes::Regex::new(r"(?P<first>\w+)\s+(?P<last>\w+)").unwrap();
    let mut locs = re.capture_locations();
    re.captures_read(&mut locs, haystack).unwrap();

    let _ = locs.get(0);
    let _ = locs.get(1);
    let _ = locs.get(2);
}

#[test]
fn test_get_capture_group_no_matches() {
    let haystack = b"This has no matches";
    let re = regex::bytes::Regex::new(r"(?P<first>\d+)\s+(?P<last>\d+)").unwrap();
    let mut locs = re.capture_locations();
    re.captures_read(&mut locs, haystack).unwrap();

    let _ = locs.get(0);
    let _ = locs.get(1);
    let _ = locs.get(2);
}

#[test]
fn test_get_negative_index() {
    let haystack = b"Bruce Springsteen";
    let re = regex::bytes::Regex::new(r"(?P<first>\w+)\s+(?P<last>\w+)").unwrap();
    let mut locs = re.capture_locations();
    re.captures_read(&mut locs, haystack).unwrap();

    let _ = locs.get(usize::MAX);
}

