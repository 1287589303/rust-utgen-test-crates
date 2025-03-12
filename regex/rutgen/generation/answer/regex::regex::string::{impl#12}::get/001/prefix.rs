// Answer 0

#[test]
fn test_get_valid_capture_index_zero() {
    let re = regex::Regex::new(r"(\d+)").unwrap();
    let caps = re.captures("123").unwrap();
    let _match = caps.get(0);
}

#[test]
fn test_get_valid_capture_index_one() {
    let re = regex::Regex::new(r"(\d+)").unwrap();
    let caps = re.captures("123").unwrap();
    let _match = caps.get(1);
}

#[test]
fn test_get_invalid_capture_index_nonexistent() {
    let re = regex::Regex::new(r"(\d+)").unwrap();
    let caps = re.captures("123").unwrap();
    let _match = caps.get(2);
}

#[test]
fn test_get_valid_capture_index_boundary() {
    let re = regex::Regex::new(r"(\d+)([a-z]*)").unwrap();
    let caps = re.captures("123abc").unwrap();
    let _match = caps.get(0);
    let _match = caps.get(1);
    let _match = caps.get(2);
}

#[test]
fn test_get_invalid_capture_index_out_of_bounds() {
    let re = regex::Regex::new(r"(\d+)").unwrap();
    let caps = re.captures("123").unwrap();
    let _match = caps.get(3);
}

