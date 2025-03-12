// Answer 0

#[test]
fn test_get_capture_group_zero() {
    use regex::bytes::Regex;
    let re = Regex::new(r"[a-z]+(?:([0-9]+)|([A-Z]+))").unwrap();
    let caps = re.captures(b"abc123").unwrap();
    let capture = caps.get(0);
}

#[test]
fn test_get_first_capture_group() {
    use regex::bytes::Regex;
    let re = Regex::new(r"[a-z]+(?:([0-9]+)|([A-Z]+))").unwrap();
    let caps = re.captures(b"abc123").unwrap();
    let capture = caps.get(1);
}

#[test]
fn test_get_second_capture_group() {
    use regex::bytes::Regex;
    let re = Regex::new(r"[a-z]+(?:([0-9]+)|([A-Z]+))").unwrap();
    let caps = re.captures(b"abc123").unwrap();
    let capture = caps.get(2);
}

#[test]
fn test_get_nonexistent_capture_group() {
    use regex::bytes::Regex;
    let re = Regex::new(r"[a-z]+(?:([0-9]+)|([A-Z]+))").unwrap();
    let caps = re.captures(b"abc123").unwrap();
    let capture = caps.get(3);
}

#[test]
fn test_get_capture_group_non_matching_input() {
    use regex::bytes::Regex;
    let re = Regex::new(r"[a-z]+(?:([0-9]+)|([A-Z]+))").unwrap();
    let caps = re.captures(b"no_match_here").unwrap();
    let capture = caps.get(0);
    let capture1 = caps.get(1);
    let capture2 = caps.get(2);
}

#[test]
fn test_get_empty_match() {
    use regex::bytes::Regex;
    let re = Regex::new(r"").unwrap();
    let caps = re.captures(b"").unwrap();
    let capture = caps.get(0);
}

