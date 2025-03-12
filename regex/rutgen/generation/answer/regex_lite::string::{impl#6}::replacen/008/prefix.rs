// Answer 0

#[test]
fn test_replacen_with_valid_regex_and_matches() {
    let re = Regex::new(r"\w+").unwrap();
    let hay = "Hello World Hello";
    let limit = 2;
    let rep = "Hi";
    let result = re.replacen(hay, limit, rep);
}

#[test]
fn test_replacen_with_multiple_matches() {
    let re = Regex::new(r"\d+").unwrap();
    let hay = "123 456 789";
    let limit = 2;
    let rep = "NUMBER";
    let result = re.replacen(hay, limit, rep);
}

#[test]
fn test_replacen_with_edge_case_limit() {
    let re = Regex::new(r"\s+").unwrap();
    let hay = "   leading spaces and trailing spaces   ";
    let limit = 1;
    let rep = "-";
    let result = re.replacen(hay, limit, rep);
}

#[test]
fn test_replacen_with_long_haystack() {
    let re = Regex::new(r"abc").unwrap();
    let hay = "abcabcabcabcabcabc";
    let limit = 3;
    let rep = "xyz";
    let result = re.replacen(hay, limit, rep);
}

#[test]
fn test_replacen_with_non_overlapping_matches() {
    let re = Regex::new(r"fruit").unwrap();
    let hay = "I like fruit and also fruit";
    let limit = 1;
    let rep = "vegetable";
    let result = re.replacen(hay, limit, rep);
}

