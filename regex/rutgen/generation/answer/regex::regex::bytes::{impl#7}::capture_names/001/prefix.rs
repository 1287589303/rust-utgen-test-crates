// Answer 0

#[test]
fn test_capture_names_with_named_and_unnamed_groups() {
    let re = Regex::new(r"(?<a>.(?<b>.))(.)(?:.)(?<c>.)").unwrap();
    let mut names = re.capture_names();
    names.next();
    names.next();
    names.next();
    names.next();
    names.next();
}

#[test]
fn test_capture_names_with_empty_regex() {
    let re = Regex::new(r"").unwrap();
    let mut names = re.capture_names();
    names.next();
    names.next();
}

#[test]
fn test_capture_names_with_no_capture_groups() {
    let re = Regex::new(r"[a&&b]").unwrap();
    let mut names = re.capture_names();
    names.next();
    names.next();
}

#[test]
fn test_capture_names_with_single_named_group() {
    let re = Regex::new(r"(?<name>abc)").unwrap();
    let mut names = re.capture_names();
    names.next();
    names.next();
}

#[test]
fn test_capture_names_with_multiple_named_groups() {
    let re = Regex::new(r"(?<first>foo)(?<second>bar)").unwrap();
    let mut names = re.capture_names();
    names.next();
    names.next();
    names.next();
}

#[test]
fn test_capture_names_with_single_unnamed_group() {
    let re = Regex::new(r"(abc)").unwrap();
    let mut names = re.capture_names();
    names.next();
    names.next();
}

#[test]
fn test_capture_names_with_only_non_capturing_group() {
    let re = Regex::new(r"(?:abc)(def)").unwrap();
    let mut names = re.capture_names();
    names.next();
    names.next();
}

