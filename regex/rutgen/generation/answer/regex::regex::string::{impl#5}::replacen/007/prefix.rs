// Answer 0

#[test]
fn test_replacen_with_limits_and_replacement() {
    let re = Regex::new(r"hello").unwrap();
    let haystack = "hello world, hello universe";
    let limit = 2;
    let replacement = "hi";
    let result = re.replacen(haystack, limit, replacement);
}

#[test]
fn test_replacen_with_different_replacement() {
    let re = Regex::new(r"dog").unwrap();
    let haystack = "dog and dog bark";
    let limit = 1;
    let replacement = "cat";
    let result = re.replacen(haystack, limit, replacement);
}

#[test]
fn test_replacen_with_multiple_matches() {
    let re = Regex::new(r"foo").unwrap();
    let haystack = "foo bar foo baz foo";
    let limit = 3;
    let replacement = "bar";
    let result = re.replacen(haystack, limit, replacement);
}

#[test]
fn test_replacen_with_haystack_ending_in_match() {
    let re = Regex::new(r"test").unwrap();
    let haystack = "this is a test test";
    let limit = 2;
    let replacement = "exam";
    let result = re.replacen(haystack, limit, replacement);
}

#[test]
fn test_replacen_with_no_expansion_replacement() {
    let re = Regex::new(r"\d+").unwrap();
    let haystack = "1 2 3 4 5";
    let limit = 4;
    let replacement = "number";
    let result = re.replacen(haystack, limit, replacement);
}

