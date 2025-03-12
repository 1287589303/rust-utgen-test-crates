// Answer 0

#[test]
fn test_len_with_valid_capture_groups() {
    let re = regex::bytes::Regex::new(r"(\w)(\d)?(\w)").unwrap();
    let caps = re.captures(b"ABC123").unwrap();
    let _ = caps.len(); // Should be 4

    let caps = re.captures(b"A").unwrap();
    let _ = caps.len(); // Should be 4

    let caps = re.captures(b"XYZ!!").unwrap();
    let _ = caps.len(); // Should be 4

    let caps = re.captures(b"123").unwrap();
    let _ = caps.len(); // Should be 4
}

#[test]
fn test_len_with_empty_input() {
    let re = regex::bytes::Regex::new(r"(\w)(\d)?(\w)").unwrap();
    let caps = re.captures(b"").unwrap();
    let _ = caps.len(); // Should be 4, includes the whole match
}

#[test]
fn test_len_with_non_matching_chars() {
    let re = regex::bytes::Regex::new(r"(\w)(\d)?(\w)").unwrap();
    let caps = re.captures(b"ZZZ").unwrap();
    let _ = caps.len(); // Should be 4, includes the whole match
}

