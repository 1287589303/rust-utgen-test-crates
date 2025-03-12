// Answer 0

#[test]
fn test_name_valid_capture_group_numbers() {
    let re = regex::Regex::new(r"[a-z]+(?:(?<numbers>[0-9]+)|(?<letters>[A-Z]+))").unwrap();
    let caps = re.captures("abc123").unwrap();
    let result = caps.name("numbers");
}

#[test]
fn test_name_valid_capture_group_letters() {
    let re = regex::Regex::new(r"[a-z]+(?:(?<numbers>[0-9]+)|(?<letters>[A-Z]+))").unwrap();
    let caps = re.captures("ABC").unwrap();
    let result = caps.name("letters");
}

#[test]
fn test_name_invalid_capture_group() {
    let re = regex::Regex::new(r"[a-z]+(?:(?<numbers>[0-9]+)|(?<letters>[A-Z]+))").unwrap();
    let caps = re.captures("abc123").unwrap();
    let result = caps.name("invalid_group");
}

#[test]
fn test_name_no_match() {
    let re = regex::Regex::new(r"[a-z]+(?:(?<numbers>[0-9]+)|(?<letters>[A-Z]+))").unwrap();
    let caps = re.captures("abc").unwrap();
    let result = caps.name("numbers");
}

#[test]
fn test_name_empty_haystack() {
    let re = regex::Regex::new(r"[a-z]+(?:(?<numbers>[0-9]+)|(?<letters>[A-Z]+))").unwrap();
    let caps = re.captures("").unwrap();
    let result = caps.name("numbers");
}

#[test]
fn test_name_multple_matches() {
    let re = regex::Regex::new(r"(?<digit>[0-9])").unwrap();
    let caps = re.captures("123").unwrap();
    let result = caps.name("digit");
}

