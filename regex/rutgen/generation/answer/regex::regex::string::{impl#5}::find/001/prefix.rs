// Answer 0

#[test]
fn test_find_valid_match() {
    let re = Regex::new(r"\b\w{13}\b").unwrap();
    let hay = "I categorically deny having triskaidekaphobia.";
    let mat = re.find(hay);
}

#[test]
fn test_find_no_match_empty_haystack() {
    let re = Regex::new(r"\b\w{13}\b").unwrap();
    let hay = "";
    let mat = re.find(hay);
}

#[test]
fn test_find_no_match_no_matches() {
    let re = Regex::new(r"\b\w{13}\b").unwrap();
    let hay = "short";
    let mat = re.find(hay);
}

#[test]
fn test_find_multiple_matches() {
    let re = Regex::new(r"\b\w{3}\b").unwrap();
    let hay = "bat cat hat";
    let mat = re.find(hay);
}

#[test]
fn test_find_boundary_case_short_string() {
    let re = Regex::new(r"\b\w{2}\b").unwrap();
    let hay = "ab";
    let mat = re.find(hay);
}

#[test]
fn test_find_boundary_case_no_match_short() {
    let re = Regex::new(r"\b\w{3}\b").unwrap();
    let hay = "ab";
    let mat = re.find(hay);
}

#[test]
fn test_find_boundary_case_long_string() {
    let re = Regex::new(r"\b\w{13}\b").unwrap();
    let hay = "a".repeat(10_001);
    let mat = re.find(&hay);
}

#[test]
fn test_find_boundary_case_just_below_match_length() {
    let re = Regex::new(r"\b\w{13}\b").unwrap();
    let hay = "abcdefghijklm";
    let mat = re.find(hay);
}

#[test]
fn test_find_boundary_case_just_above_match_length() {
    let re = Regex::new(r"\b\w{13}\b").unwrap();
    let hay = "abcdefghijklmn";
    let mat = re.find(hay);
}

