// Answer 0

#[test]
fn test_len_multiple_capture_groups() {
    let pattern = r"(\w)(\d)?(\w)";
    let re = regex_lite::Regex::new(pattern).unwrap();
    let caps = re.captures("XYZ123").unwrap();
    let _ = caps.len();
}

#[test]
fn test_len_single_capture_group() {
    let pattern = r"(\w+)";
    let re = regex_lite::Regex::new(pattern).unwrap();
    let caps = re.captures("A").unwrap();
    let _ = caps.len();
}

#[test]
fn test_len_no_capture_groups() {
    let pattern = r"()";
    let re = regex_lite::Regex::new(pattern).unwrap();
    let caps = re.captures("A").unwrap();
    let _ = caps.len();
}

#[test]
fn test_len_all_capture_groups_optional() {
    let pattern = r"(\d*)(\w*)";
    let re = regex_lite::Regex::new(pattern).unwrap();
    let caps = re.captures("12").unwrap();
    let _ = caps.len();
}

#[test]
fn test_len_empty_string() {
    let pattern = r"(\w*)";
    let re = regex_lite::Regex::new(pattern).unwrap();
    let caps = re.captures("").unwrap();
    let _ = caps.len();
}

