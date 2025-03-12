// Answer 0

#[test]
fn test_replace_empty_haystack() {
    let re = Regex::new(r"a").unwrap();
    let result = re.replace("", "b");
}

#[test]
fn test_replace_no_matches() {
    let re = Regex::new(r"a").unwrap();
    let result = re.replace("bcd", "x");
}

#[test]
fn test_replace_with_empty_string() {
    let re = Regex::new(r"a").unwrap();
    let result = re.replace("abc", "");
}

#[test]
fn test_replace_with_simple_capture() {
    let re = Regex::new(r"(?P<name>\w+)").unwrap();
    let result = re.replace("John", "${name}");
}

#[test]
fn test_replace_with_special_characters() {
    let re = Regex::new(r"(\W)").unwrap();
    let result = re.replace("Hello, World!", "_");
}

#[test]
fn test_replace_with_capture_function() {
    let re = Regex::new(r"(?P<first>\w+) (?P<last>\w+)").unwrap();
    let result = re.replace("John Doe", |caps: &Captures| {
        format!("{} {}", &caps["last"], &caps["first"])
    });
}

#[test]
fn test_replace_with_limit_exceeded() {
    let re = Regex::new(r"a").unwrap();
    let result = re.replace("aaaaaa", "b");
}

#[test]
fn test_replace_with_named_capture_only() {
    let re = Regex::new(r"(?P<name>\w+)").unwrap();
    let result = re.replace("Alice", "${name}");
}

#[test]
fn test_replace_with_no_expand() {
    use regex::NoExpand;
    let re = Regex::new(r"(?P<last>\w+),\s+(\S+)").unwrap();
    let result = re.replace("Springsteen, Bruce", NoExpand("$2 $last"));
}

#[test]
fn test_replace_long_haystack() {
    let re = Regex::new(r"the").unwrap();
    let result = re.replace("the quick brown fox jumps over the lazy dog", "a");
}

#[test]
fn test_replace_with_large_limit() {
    let re = Regex::new(r"a").unwrap();
    let result = re.replace("aaaaa", "b");
}

