// Answer 0

#[test]
fn test_replacen_with_limit_and_no_expansion() {
    use regex::bytes::Regex;
    use alloc::borrow::Cow;

    let re = Regex::new(r"(?m)\b(\w+)\b").unwrap(); // Simple word boundary regex
    let haystack = b"apple banana apple orange apple"; // Contains multiple matches for "apple"
    let limit = 2; // Replace first 2 occurrences
    let replacement = b"fruit"; // Replacement without capture expansions

    let result = re.replacen(haystack, limit, replacement);
}

#[test]
fn test_replacen_with_no_match_found() {
    use regex::bytes::Regex;
    use alloc::borrow::Cow;

    let re = Regex::new(r"(?m)grape").unwrap(); // Regex that doesn't match the haystack
    let haystack = b"apple banana apple orange apple"; // No "grape" present
    let limit = 1; // Limited to 1
    let replacement = b"fruit"; // Replacement without capture expansions

    let result = re.replacen(haystack, limit, replacement);
}

#[test]
fn test_replacen_with_exact_limit() {
    use regex::bytes::Regex;
    use alloc::borrow::Cow;

    let re = Regex::new(r"(?m)\bapple\b").unwrap(); // Regex to match "apple"
    let haystack = b"apple banana apple orange apple"; // Matches "apple" multiple times
    let limit = 3; // Replace first 3 occurrences
    let replacement = b"fruit"; // Replacement without capture expansions

    let result = re.replacen(haystack, limit, replacement);
}

