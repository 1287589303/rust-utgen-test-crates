// Answer 0

#[test]
fn test_replacen_with_valid_regex_no_expansion() {
    let re = Regex::new(r"\d+").unwrap();
    let haystack = "There are 123 apples and 456 oranges.";
    let limit = 1;
    let replacement = "NUMBER";
    
    let result = re.replacen(haystack, limit, replacement);
}

#[test]
fn test_replacen_with_multiple_matches_no_expansion() {
    let re = Regex::new(r"\s+").unwrap();
    let haystack = "This   is   a test.";
    let limit = 2;
    let replacement = " ";
    
    let result = re.replacen(haystack, limit, replacement);
}

#[test]
fn test_replacen_with_no_capture_groups() {
    let re = Regex::new(r"cat").unwrap();
    let haystack = "cat dog cat";
    let limit = 1;
    let replacement = "feline";
    
    let result = re.replacen(haystack, limit, replacement);
}

#[test]
fn test_replacen_with_exactly_one_match() {
    let re = Regex::new(r"foo").unwrap();
    let haystack = "foo bar baz";
    let limit = 1;
    let replacement = "replace";
    
    let result = re.replacen(haystack, limit, replacement);
}

