// Answer 0

#[test]
fn test_captures_len_implicit_group() {
    let re = Regex::new(r"foo").unwrap();
    let _ = re.captures_len();
}

#[test]
fn test_captures_len_single_named_group() {
    let re = Regex::new(r"(foo)").unwrap();
    let _ = re.captures_len();
}

#[test]
fn test_captures_len_multiple_named_groups() {
    let re = Regex::new(r"(?<a>.(?<b>.))(.)(?:.)(?<c>.)").unwrap();
    let _ = re.captures_len();
}

#[test]
fn test_captures_len_intersection_pattern() {
    let re = Regex::new(r"[a&&b]").unwrap();
    let _ = re.captures_len();
}

#[test]
fn test_captures_len_empty_regex() {
    let re = Regex::new(r"").unwrap();
    let _ = re.captures_len();
}

#[test]
fn test_captures_len_non_capturing_group() {
    let re = Regex::new(r"(?:test)").unwrap();
    let _ = re.captures_len();
}

#[test]
#[should_panic]
fn test_captures_len_malformed_regex() {
    let _ = Regex::new(r"(");
}

#[test]
fn test_captures_len_nested_groups() {
    let re = Regex::new(r"((a)(b)(c))").unwrap();
    let _ = re.captures_len();
}

#[test]
fn test_captures_len_no_groups() {
    let re = Regex::new(r"^\d+$").unwrap();
    let _ = re.captures_len();
}

#[test]
fn test_captures_len_multiple_unnamed_groups() {
    let re = Regex::new(r"(foo)(bar)(baz)").unwrap();
    let _ = re.captures_len();
}

