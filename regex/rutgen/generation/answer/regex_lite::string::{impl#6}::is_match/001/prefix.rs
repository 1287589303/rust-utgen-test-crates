// Answer 0

#[test]
fn test_is_match_empty_haystack() {
    let re = regex_lite::Regex::new(r"\d+").unwrap();
    let result = re.is_match("");
}

#[test]
fn test_is_match_no_match() {
    let re = regex_lite::Regex::new(r"abc").unwrap();
    let result = re.is_match("xyz");
}

#[test]
fn test_is_match_single_word() {
    let re = regex_lite::Regex::new(r"\w+").unwrap();
    let result = re.is_match("hello");
}

#[test]
fn test_is_match_multiple_matches() {
    let re = regex_lite::Regex::new(r"\d+").unwrap();
    let result = re.is_match("There are 2 cats and 3 dogs.");
}

#[test]
fn test_is_match_exact_length() {
    let re = regex_lite::Regex::new(r"\b\w{13}\b").unwrap();
    let result = re.is_match("I categorically deny having triskaidekaphobia.");
}

#[test]
fn test_is_match_large_haystack() {
    let re = regex_lite::Regex::new(r"\b\w{5}\b").unwrap();
    let large_haystack = "a ".repeat(1_000_000) + "apple"; // large string with a match
    let result = re.is_match(&large_haystack);
}

#[test]
fn test_is_match_complex_pattern() {
    let re = regex_lite::Regex::new(r"(\w+)-(\w+)").unwrap();
    let result = re.is_match("hello-world");
}

#[test]
fn test_is_match_boundary_case() {
    let re = regex_lite::Regex::new(r"\w{1,20}").unwrap();
    let result = re.is_match("a"); // minimum length
}

