// Answer 0

#[test]
fn test_capture_locations_valid_pattern() {
    let re = Regex::new(r"(.)(.)(\w+)").unwrap();
    let locs = re.capture_locations();
}

#[test]
fn test_capture_locations_empty_string() {
    let re = Regex::new(r"(.)").unwrap();
    let locs = re.capture_locations();
}

#[test]
fn test_capture_locations_single_character_input() {
    let re = Regex::new(r"(\w)").unwrap();
    let locs = re.capture_locations();
}

#[test]
fn test_capture_locations_multiple_byte_input() {
    let re = Regex::new(r"(.)(.)(\w{1,100})").unwrap();
    let locs = re.capture_locations();
}

