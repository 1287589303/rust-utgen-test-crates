// Answer 0

#[test]
fn test_name_valid_numbers_capture() {
    let re = regex::bytes::Regex::new(r"[a-z]+(?:(?<numbers>[0-9]+)|(?<letters>[A-Z]+))").unwrap();
    let caps = re.captures(b"abc123").unwrap();
    let numbers = caps.name("numbers");
}

#[test]
fn test_name_valid_letters_capture() {
    let re = regex::bytes::Regex::new(r"[a-z]+(?:(?<numbers>[0-9]+)|(?<letters>[A-Z]+))").unwrap();
    let caps = re.captures(b"abcXYZ").unwrap();
    let letters = caps.name("letters");
}

#[test]
fn test_name_no_capture_match() {
    let re = regex::bytes::Regex::new(r"[a-z]+(?:(?<numbers>[0-9]+)|(?<letters>[A-Z]+))").unwrap();
    let caps = re.captures(b"abc").unwrap();
    let letters = caps.name("numbers");
}

#[test]
fn test_name_invalid_capture_group() {
    let re = regex::bytes::Regex::new(r"[a-z]+(?:(?<numbers>[0-9]+)|(?<letters>[A-Z]+))").unwrap();
    let caps = re.captures(b"abc123").unwrap();
    let non_existent = caps.name("non_existent_group");
}

#[test]
fn test_name_empty_haystack() {
    let re = regex::bytes::Regex::new(r"[a-z]+(?:(?<numbers>[0-9]+)|(?<letters>[A-Z]+))").unwrap();
    let caps = re.captures(b"").unwrap();
    let letters = caps.name("numbers");
}

