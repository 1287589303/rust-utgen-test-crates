// Answer 0

#[test]
fn test_replace_all_with_empty_haystack() {
    let re = Regex::new(r"\w+").unwrap();
    let result = re.replace_all("", "replacement");
}

#[test]
fn test_replace_all_with_non_empty_haystack() {
    let re = Regex::new(r"\w+").unwrap();
    let result = re.replace_all("hello world", "replacement");
}

#[test]
fn test_replace_all_with_multiple_matches() {
    let re = Regex::new(r"\w+").unwrap();
    let result = re.replace_all("one two three", "replacement");
}

#[test]
fn test_replace_all_with_large_string() {
    let re = Regex::new(r"\d+").unwrap();
    let large_haystack = "1234567890".repeat(1000);
    let result = re.replace_all(&large_haystack, "replacement");
}

#[test]
fn test_replace_all_with_long_word() {
    let re = Regex::new(r"\w+").unwrap();
    let result = re.replace_all("longword replacement test", "replaced");
}

#[test]
fn test_replace_all_with_pattern_matching_at_start() {
    let re = Regex::new(r"^start").unwrap();
    let result = re.replace_all("start of the string", "beginning");
}

#[test]
fn test_replace_all_with_pattern_matching_at_end() {
    let re = Regex::new(r"end$").unwrap();
    let result = re.replace_all("the end", "the finish");
}

