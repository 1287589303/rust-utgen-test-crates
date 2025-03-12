// Answer 0

#[test]
fn test_captures_iter_valid_pattern_and_haystack() {
    let re = Regex::new(r"'([^']+)'\s+\(([0-9]{4})\)").unwrap();
    let haystack = "'Citizen Kane' (1941), 'The Wizard of Oz' (1939), 'M' (1931).";
    let captures = re.captures_iter(haystack);
}

#[test]
fn test_captures_iter_empty_haystack() {
    let re = Regex::new(r"'([^']+)'\s+\(([0-9]{4})\)").unwrap();
    let haystack = "";
    let captures = re.captures_iter(haystack);
}

#[test]
fn test_captures_iter_no_matches() {
    let re = Regex::new(r"'([^']+)'\s+\(([0-9]{4})\)").unwrap();
    let haystack = "'Not a match' (abcd)";
    let captures = re.captures_iter(haystack);
}

#[test]
fn test_captures_iter_with_named_groups() {
    let re = Regex::new(r"'(?<title>[^']+)'\s+\((?<year>[0-9]{4})\)").unwrap();
    let haystack = "'Citizen Kane' (1941), 'The Wizard of Oz' (1939)";
    let captures = re.captures_iter(haystack);
}

#[test]
fn test_captures_iter_pattern_with_no_capture_groups() {
    let re = Regex::new(r"\d+").unwrap();
    let haystack = "There are 123 and 456.";
    let captures = re.captures_iter(haystack);
}

