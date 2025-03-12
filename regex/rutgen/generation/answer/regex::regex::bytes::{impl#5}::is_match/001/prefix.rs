// Answer 0

#[test]
fn test_is_match_valid_regex_pattern_with_match() {
    let re = Regex::new(r"\b\w{3}\b").unwrap();
    let hay = b"The fox jumps over the lazy dog.";
    let _ = re.is_match(hay);
}

#[test]
fn test_is_match_valid_regex_pattern_with_no_match() {
    let re = Regex::new(r"\b\w{5}\b").unwrap();
    let hay = b"It is a sunny day.";
    let _ = re.is_match(hay);
}

#[test]
fn test_is_match_empty_haystack() {
    let re = Regex::new(r"\b\w{3}\b").unwrap();
    let hay: &[u8] = b"";
    let _ = re.is_match(hay);
}

#[test]
fn test_is_match_haystack_with_special_characters() {
    let re = Regex::new(r"\w+").unwrap();
    let hay = b"Hello, world! @2023.";
    let _ = re.is_match(hay);
}

#[test]
fn test_is_match_haystack_with_unicode_characters() {
    let re = Regex::new(r"\w{5}").unwrap();
    let hay = "こんにちは世界".as_bytes();
    let _ = re.is_match(hay);
}

#[test]
fn test_is_match_long_regex_pattern_with_match() {
    let re = Regex::new(r"\b\w{37}\b").unwrap();
    let hay = b"The quick brown fox jumps over the lazy dog which is one very special dog.";
    let _ = re.is_match(hay);
}

#[test]
fn test_is_match_boundary_case_max_haystack_length() {
    let re = Regex::new(r"\w{50}").unwrap();
    let hay = b"A"*1000; // 1000 bytes with repeated 'A'
    let _ = re.is_match(hay);
}

#[test]
fn test_is_match_boundary_case_max_regex_length() {
    let re = Regex::new(r"\w{50}").unwrap();
    let hay = b"This is a test string with no pattern match here.";
    let _ = re.is_match(hay);
}

