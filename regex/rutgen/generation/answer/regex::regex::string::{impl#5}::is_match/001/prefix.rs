// Answer 0

#[test]
fn test_is_match_empty_string() {
    let re = Regex::new(r"\b\w{13}\b").unwrap();
    let hay = "";
    re.is_match(hay);
}

#[test]
fn test_is_match_no_match() {
    let re = Regex::new(r"\b\w{13}\b").unwrap();
    let hay = "This string has no matching words.";
    re.is_match(hay);
}

#[test]
fn test_is_match_one_match() {
    let re = Regex::new(r"\b\w{13}\b").unwrap();
    let hay = "This string has exactly thirteen characters.";
    re.is_match(hay);
}

#[test]
fn test_is_match_multiple_matches() {
    let re = Regex::new(r"\b\w{13}\b").unwrap();
    let hay = "I have thirteen characters and another thirteen.";
    re.is_match(hay);
}

#[test]
fn test_is_match_special_characters() {
    let re = Regex::new(r"\b\w{13}\b").unwrap();
    let hay = "This! Has? Thirteen... Characters.";
    re.is_match(hay);
}

#[test]
fn test_is_match_unicode_characters() {
    let re = Regex::new(r"\b\w{13}\b").unwrap();
    let hay = "This is a test 𝕓𝕖𝕒𝕣𝕤𝕖𝕣 with 13 characters.";
    re.is_match(hay);
}

#[test]
fn test_is_match_boundary_length() {
    let re = Regex::new(r"\b\w{1}\b").unwrap();
    let hay = "a"; // single character match
    re.is_match(hay);
}

