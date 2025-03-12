// Answer 0

#[test]
fn test_len_empty_string() {
    let re = regex::Regex::new(r"(\w)(\d)?(\w)").unwrap();
    let caps = re.captures("").unwrap();
    let _ = caps.len();
}

#[test]
fn test_len_no_matches() {
    let re = regex::Regex::new(r"(\w)(\d)?(\w)").unwrap();
    let caps = re.captures("123456").unwrap();
    let _ = caps.len();
}

#[test]
fn test_len_one_match() {
    let re = regex::Regex::new(r"(\w)(\d)?(\w)").unwrap();
    let caps = re.captures("A3B").unwrap();
    let _ = caps.len();
}

#[test]
fn test_len_multiple_matches() {
    let re = regex::Regex::new(r"(\w)(\d)?(\w)").unwrap();
    let caps = re.captures("A1B A2C A3D").unwrap();
    let _ = caps.len();
}

#[test]
fn test_len_partial_match() {
    let re = regex::Regex::new(r"(\w)(\d)?(\w)").unwrap();
    let caps = re.captures("AZ").unwrap();
    let _ = caps.len();
}

#[test]
fn test_len_full_match() {
    let re = regex::Regex::new(r"(\w)(\d)?(\w)").unwrap();
    let caps = re.captures("A1B").unwrap();
    let _ = caps.len();
}

