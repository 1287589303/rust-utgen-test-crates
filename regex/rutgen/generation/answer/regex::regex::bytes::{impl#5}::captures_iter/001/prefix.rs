// Answer 0

#[test]
fn test_captures_iter_single_match() {
    let re = Regex::new(r"'([^']+)'\s+\(([0-9]{4})\)").unwrap();
    let haystack = b"'The Matrix' (1999)";
    let _captures = re.captures_iter(haystack);
}

#[test]
fn test_captures_iter_multiple_matches() {
    let re = Regex::new(r"'([^']+)'\s+\(([0-9]{4})\)").unwrap();
    let haystack = b"'Inception' (2010), 'Interstellar' (2014)";
    let _captures = re.captures_iter(haystack);
}

#[test]
fn test_captures_iter_no_match() {
    let re = Regex::new(r"'([^']+)'\s+\(([0-9]{4})\)").unwrap();
    let haystack = b"No matches here!";
    let _captures = re.captures_iter(haystack);
}

#[test]
fn test_captures_iter_empty_haystack() {
    let re = Regex::new(r"'([^']+)'\s+\(([0-9]{4})\)").unwrap();
    let haystack: &[u8] = b"";
    let _captures = re.captures_iter(haystack);
}

#[test]
fn test_captures_iter_non_utf8_haystack() {
    let re = Regex::new(r"'([^']+)'\s+\(([0-9]{4})\)").unwrap();
    let haystack = b"\xFFInvalid\xFF";
    let _captures = re.captures_iter(haystack);
}

#[test]
fn test_captures_iter_exact_match() {
    let re = Regex::new(r"'([^']+)'\s+\(([0-9]{4})\)").unwrap();
    let haystack = b"'Pulp Fiction' (1994)";
    let _captures = re.captures_iter(haystack);
}

#[test]
fn test_captures_iter_haystack_with_full_match() {
    let re = Regex::new(r"^(.*)$").unwrap();
    let haystack = b"Entire haystack";
    let _captures = re.captures_iter(haystack);
}

#[test]
fn test_captures_iter_boundary_case_long_haystack() {
    let re = Regex::new(r"'([^']+)'\s+\(([0-9]{4})\)").unwrap();
    let haystack = b"'Title' (2021) ".repeat(70); // Approximately 1000 bytes
    let _captures = re.captures_iter(haystack);
}

#[test]
fn test_captures_iter_boundary_case_short_haystack() {
    let re = Regex::new(r"'([^']+)'\s+\(([0-9]{4})\)").unwrap();
    let haystack = b"'A' (1)";
    let _captures = re.captures_iter(haystack);
}

