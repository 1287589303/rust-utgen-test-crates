// Answer 0

#[test]
fn test_capture_locations_len_multi_groups() {
    use crate::Regex;

    let re = Regex::new(r"(?<first>\w+)\s+(?<last>\w+)").unwrap();
    let locs = re.capture_locations();
    locs.len();
}

#[test]
fn test_capture_locations_len_empty_pattern() {
    use crate::Regex;

    let re = Regex::new(r"").unwrap();
    let locs = re.capture_locations();
    locs.len();
}

#[test]
fn test_capture_locations_len_non_matching_pattern() {
    use crate::Regex;

    let re = Regex::new(r"[^\s\S]").unwrap();
    let locs = re.capture_locations();
    locs.len();
}

