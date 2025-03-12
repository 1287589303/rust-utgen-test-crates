// Answer 0

#[test]
fn test_captures_read_at_valid_start() {
    let re = Regex::new(r"\bchew\b").unwrap();
    let hay = "eschew";
    let mut locs = re.capture_locations();

    let result = re.captures_read_at(&mut locs, hay, 0);
}

#[test]
fn test_captures_read_at_mid_point() {
    let re = Regex::new(r"\bchew\b").unwrap();
    let hay = "eschew";
    let mut locs = re.capture_locations();

    let result = re.captures_read_at(&mut locs, hay, 2);
}

#[test]
fn test_captures_read_at_end() {
    let re = Regex::new(r"\bchew\b").unwrap();
    let hay = "eschew";
    let mut locs = re.capture_locations();

    let result = re.captures_read_at(&mut locs, hay, hay.len() - 3);
}

#[test]
#[should_panic]
fn test_captures_read_at_panic_start_exceeds_length() {
    let re = Regex::new(r"\bchew\b").unwrap();
    let hay = "eschew";
    let mut locs = re.capture_locations();

    let result = re.captures_read_at(&mut locs, hay, hay.len());
}

#[test]
fn test_captures_read_at_edge_case_empty_haystack() {
    let re = Regex::new(r"\bchew\b").unwrap();
    let hay = "";
    let mut locs = re.capture_locations();

    let result = re.captures_read_at(&mut locs, hay, 0);
}

