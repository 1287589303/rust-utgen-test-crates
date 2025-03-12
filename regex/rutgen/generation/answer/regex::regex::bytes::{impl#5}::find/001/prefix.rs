// Answer 0

#[test]
fn test_find_valid_match() {
    let re = Regex::new(r"\b\w{13}\b").unwrap();
    let hay = b"I categorically deny having triskaidekaphobia.";
    re.find(hay).unwrap();
}

#[test]
fn test_find_empty_haystack() {
    let re = Regex::new(r"\b\w{13}\b").unwrap();
    let hay: &[u8] = b"";
    let result = re.find(hay);
}

#[test]
fn test_find_no_matches() {
    let re = Regex::new(r"\b\w{10}\b").unwrap();
    let hay = b"I categorically deny having triskaidekaphobia.";
    let result = re.find(hay);
}

#[test]
fn test_find_multiple_matches() {
    let re = Regex::new(r"\b\w{5}\b").unwrap();
    let hay = b"The quick brown fox jumps over the lazy dogs.";
    re.find(hay).unwrap();
}

#[test]
fn test_find_match_at_boundary() {
    let re = Regex::new(r"\bfox\b").unwrap();
    let hay = b"The quick brown fox.";
    re.find(hay).unwrap();
}

#[test]
fn test_find_match_at_beginning() {
    let re = Regex::new(r"\blazy\b").unwrap();
    let hay = b"Lazily does the dog doze.";
    re.find(hay).unwrap();
}

#[test]
fn test_find_match_at_end() {
    let re = Regex::new(r"\bthe\b").unwrap();
    let hay = b"The cat sat on the mat.";
    re.find(hay).unwrap();
}

#[test]
fn test_find_long_haystack() {
    let re = Regex::new(r"\b\w{6,}\b").unwrap();
    let hay = b"This is a long string with several longer words.";
    re.find(hay).unwrap();
}

#[test]
fn test_find_special_characters() {
    let re = Regex::new(r"\d{3}").unwrap();
    let hay = b"Num: 123, Num: 456, Num: 789";
    re.find(hay).unwrap();
}

