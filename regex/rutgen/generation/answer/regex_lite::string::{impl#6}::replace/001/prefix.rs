// Answer 0

#[test]
fn test_replace_with_empty_haystack() {
    let re = Regex::new(r"\d+").unwrap();
    let result = re.replace("", "replacement");
}

#[test]
fn test_replace_with_no_match() {
    let re = Regex::new(r"\d+").unwrap();
    let result = re.replace("abcdef", "replacement");
}

#[test]
fn test_replace_with_single_match() {
    let re = Regex::new(r"\d+").unwrap();
    let result = re.replace("abc123xyz", "number");
}

#[test]
fn test_replace_with_multiple_matches() {
    let re = Regex::new(r"\d+").unwrap();
    let result = re.replace("123abc456def", "num");
}

#[test]
fn test_replace_with_named_capture_group() {
    let re = Regex::new(r"(?<first>\w+)\s+(?<second>\w+)").unwrap();
    let result = re.replace("John Doe", "$first $second");
}

#[test]
fn test_replace_with_no_expand() {
    use regex_lite::NoExpand;
    let re = Regex::new(r"(?<last>[^,\s]+),\s+(\S+)").unwrap();
    let result = re.replace("Springsteen, Bruce", NoExpand("$2 $last"));
}

#[test]
fn test_replace_with_invalid_capture_group() {
    let re = Regex::new(r"(?<name>\w+)").unwrap();
    let result = re.replace("Alice", "$invalid");
}

#[test]
fn test_replace_with_braces() {
    let re = Regex::new(r"(?<number>\d+)").unwrap();
    let result = re.replace("Number 42", "${number} is the answer");
}

#[test]
fn test_replace_with_limit() {
    let re = Regex::new(r"(\d+)").unwrap();
    let result = re.replace("1 2 3 4", "number");
}

#[test]
fn test_replace_with_no_matches_and_no_replacement() {
    let re = Regex::new(r"xyz").unwrap();
    let result = re.replace("abc", "");
}

