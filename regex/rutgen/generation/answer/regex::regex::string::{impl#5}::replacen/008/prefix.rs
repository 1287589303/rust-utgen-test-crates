// Answer 0

#[test]
fn test_replacen_with_single_match_and_limit() {
    let re = regex::Regex::new(r"apple").unwrap();
    let haystack = "apple banana apple grape";
    let limit = 1;
    let replacement = "orange";
    let result = re.replacen(haystack, limit, replacement);
}

#[test]
fn test_replacen_with_multiple_matches_and_limit() {
    let re = regex::Regex::new(r"banana").unwrap();
    let haystack = "apple banana banana grape";
    let limit = 2;
    let replacement = "orange";
    let result = re.replacen(haystack, limit, replacement);
}

#[test]
fn test_replacen_with_no_matches() {
    let re = regex::Regex::new(r"kiwi").unwrap();
    let haystack = "apple banana grape";
    let limit = 1;
    let replacement = "orange";
    let result = re.replacen(haystack, limit, replacement);
}

#[test]
fn test_replacen_with_full_limit() {
    let re = regex::Regex::new(r"banana").unwrap();
    let haystack = "apple banana banana grape";
    let limit = 2;
    let replacement = "orange";
    let result = re.replacen(haystack, limit, replacement);
}

#[test]
fn test_replacen_with_allowance_for_limit() {
    let re = regex::Regex::new(r"banana").unwrap();
    let haystack = "apple banana banana grape";
    let limit = 2;
    let replacement = "orange";
    let result = re.replacen(haystack, limit, replacement);
}

