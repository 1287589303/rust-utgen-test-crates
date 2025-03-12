// Answer 0

#[test]
fn test_replacen_no_expansion_with_matches() {
    let re = regex::bytes::Regex::new(r"a").unwrap();
    let haystack: &[u8] = b"abcabc";
    let limit = 2;
    let replacement: &[u8] = b"x";

    let result = re.replacen(haystack, limit, replacement);
}

#[test]
fn test_replacen_no_expansion_with_full_haystack() {
    let re = regex::bytes::Regex::new(r"foo").unwrap();
    let haystack: &[u8] = b"foobarfoo";
    let limit = 1;
    let replacement: &[u8] = b"bar";

    let result = re.replacen(haystack, limit, replacement);
}

#[test]
fn test_replacen_no_expansion_multiple_matches() {
    let re = regex::bytes::Regex::new(r"cat").unwrap();
    let haystack: &[u8] = b"catdogcatmouse";
    let limit = 3;
    let replacement: &[u8] = b"dog";

    let result = re.replacen(haystack, limit, replacement);
}

#[test]
fn test_replacen_no_expansion_exact_match() {
    let re = regex::bytes::Regex::new(r"z").unwrap();
    let haystack: &[u8] = b"z";
    let limit = 1;
    let replacement: &[u8] = b"x";

    let result = re.replacen(haystack, limit, replacement);
}

#[test]
fn test_replacen_no_expansion_empty_haystack() {
    let re = regex::bytes::Regex::new(r"a").unwrap();
    let haystack: &[u8] = b"";
    let limit = 1;
    let replacement: &[u8] = b"b";

    let result = re.replacen(haystack, limit, replacement);
}

