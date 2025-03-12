// Answer 0

#[test]
fn test_len_with_valid_regex_groups() {
    use regex::bytes::Regex;

    let re = Regex::new(r"(?<first>\w+)\s+(?<last>\w+)").unwrap();
    let locs = re.capture_locations();
    locs.len();
    let mut locs = re.capture_locations();
    re.captures_read(&mut locs, b"Bruce Springsteen").unwrap();
    locs.len();
}

#[test]
fn test_len_with_empty_regex() {
    use regex::bytes::Regex;

    let re = Regex::new(r"").unwrap();
    let locs = re.capture_locations();
    locs.len();
}

#[test]
fn test_len_with_never_matching_regex() {
    use regex::bytes::Regex;

    let re = Regex::new(r"[a&&b]").unwrap();
    let locs = re.capture_locations();
    locs.len();
}

#[test]
fn test_len_with_multiple_groups() {
    use regex::bytes::Regex;

    let re = Regex::new(r"(?<year>\d{4})-(?<month>\d{2})-(?<day>\d{2})").unwrap();
    let locs = re.capture_locations();
    locs.len();
}

#[test]
fn test_len_with_no_matches() {
    use regex::bytes::Regex;

    let re = Regex::new(r"(?<name>\w+)").unwrap();
    let locs = re.capture_locations();
    let mut locs = re.capture_locations();
    re.captures_read(&mut locs, b"").unwrap();
    locs.len();
}

#[test]
fn test_len_with_large_regex() {
    use regex::bytes::Regex;

    let re = Regex::new(r"(?:(?P<large_group>\w{100}))").unwrap();
    let locs = re.capture_locations();
    locs.len();
}

