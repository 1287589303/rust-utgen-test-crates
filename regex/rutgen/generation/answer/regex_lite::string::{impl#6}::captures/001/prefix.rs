// Answer 0

#[test]
fn test_captures_valid_regex_non_empty_haystack() {
    let re = Regex::new(r"'([^']+)'\s+\((\d{4})\)").unwrap();
    let hay = "Not my favorite movie: 'Citizen Kane' (1941).";
    let _caps = re.captures(hay);
}

#[test]
fn test_captures_valid_regex_no_match_haystack() {
    let re = Regex::new(r"'([^']+)'\s+\((\d{4})\)").unwrap();
    let hay = "Just some random text.";
    let _caps = re.captures(hay);
}

#[test]
fn test_captures_empty_haystack() {
    let re = Regex::new(r"'([^']+)'\s+\((\d{4})\)").unwrap();
    let hay = "";
    let _caps = re.captures(hay);
}

#[test]
fn test_captures_empty_pattern() {
    let re = Regex::new("").unwrap();
    let hay = "Some haystack text.";
    let _caps = re.captures(hay);
}

#[test]
fn test_captures_malformed_regex() {
    let re = Regex::new(r"'([^']+)\s+\(\d{4}").unwrap(); // missing closing parenthesis
    let hay = "Some text for testing.";
    let _caps = re.captures(hay);
}

#[test]
fn test_captures_valid_regex_named_capture_groups() {
    let re = Regex::new(r"'(?<title>[^']+)'\s+\((?<year>\d{4})\)").unwrap();
    let hay = "Not my favorite movie: 'Citizen Kane' (1941).";
    let _caps = re.captures(hay);
}

#[test]
fn test_captures_whitespace_only_haystack() {
    let re = Regex::new(r"'([^']+)'\s+\((\d{4})\)").unwrap();
    let hay = "   ";
    let _caps = re.captures(hay);
}

#[test]
fn test_captures_long_haystack() {
    let re = Regex::new(r"'([^']+)'\s+\((\d{4})\)").unwrap();
    let hay = "x".repeat(1_000); // long string near maximum length limits
    let _caps = re.captures(hay.as_str());
}

