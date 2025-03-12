// Answer 0

#[test]
fn test_capture_locations_with_one_group() {
    let re = Regex {
        meta: meta::Regex::new(r"(.+)").unwrap(),
        pattern: Arc::from(r"(.+)"),
    };
    let locs = re.capture_locations();
}

#[test]
fn test_capture_locations_with_three_groups() {
    let re = Regex {
        meta: meta::Regex::new(r"(.)(.)(\w+)").unwrap(),
        pattern: Arc::from(r"(.)(.)(\w+)"),
    };
    let locs = re.capture_locations();
}

#[test]
fn test_capture_locations_with_maximum_groups() {
    let re = Regex {
        meta: meta::Regex::new(r"(.)(.)(.)(.)(.)(.)(.)(.)(.)(.)").unwrap(),
        pattern: Arc::from(r"(.)(.)(.)(.)(.)(.)(.)(.)(.)(.)"),
    };
    let locs = re.capture_locations();
}

#[test]
fn test_capture_locations_with_empty_input() {
    let re = Regex {
        meta: meta::Regex::new(r"()").unwrap(),
        pattern: Arc::from(r"()"),
    };
    let locs = re.capture_locations();
}

#[test]
fn test_capture_locations_with_full_match() {
    let re = Regex {
        meta: meta::Regex::new(r"^(.*)$").unwrap(),
        pattern: Arc::from(r"^(.*)$"),
    };
    let locs = re.capture_locations();
}

