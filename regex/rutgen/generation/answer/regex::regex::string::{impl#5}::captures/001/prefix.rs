// Answer 0

#[test]
fn test_captures_simple_pattern() {
    let re = Regex::new(r"(\w+)").unwrap();
    let hay = "hello";
    let _caps = re.captures(hay);
}

#[test]
fn test_captures_named_groups() {
    let re = Regex::new(r"(?<name>\w+) is (?<age>\d+) years old").unwrap();
    let hay = "Alice is 30 years old";
    let _caps = re.captures(hay);
}

#[test]
fn test_captures_nested_groups() {
    let re = Regex::new(r"(Hello, (\w+))").unwrap();
    let hay = "Hello, world!";
    let _caps = re.captures(hay);
}

#[test]
fn test_captures_empty_haystack() {
    let re = Regex::new(r"\w+").unwrap();
    let hay = "";
    let _caps = re.captures(hay);
}

#[test]
fn test_captures_single_character_haystack() {
    let re = Regex::new(r"a").unwrap();
    let hay = "a";
    let _caps = re.captures(hay);
}

#[test]
fn test_captures_exact_match() {
    let re = Regex::new(r"exact").unwrap();
    let hay = "exact";
    let _caps = re.captures(hay);
}

#[test]
fn test_captures_failure() {
    let re = Regex::new(r"\d+").unwrap();
    let hay = "no digits here";
    let _caps = re.captures(hay);
}

#[test]
fn test_captures_special_characters() {
    let re = Regex::new(r"[\w\s]+").unwrap();
    let hay = "Hello, World!";
    let _caps = re.captures(hay);
}

#[test]
fn test_captures_with_whitespace() {
    let re = Regex::new(r"\s+").unwrap();
    let hay = "This  is  a test.";
    let _caps = re.captures(hay);
}

