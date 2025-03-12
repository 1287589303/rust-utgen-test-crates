// Answer 0

#[test]
fn test_captures_len_simple() {
    let re = Regex::new(r"foo").unwrap();
    let _ = re.captures_len();
}

#[test]
fn test_captures_len_with_group() {
    let re = Regex::new(r"(foo)").unwrap();
    let _ = re.captures_len();
}

#[test]
fn test_captures_len_nested_groups() {
    let re = Regex::new(r"(?<a>.(?<b>.))(.)(?:.)(?<c>.)").unwrap();
    let _ = re.captures_len();
}

#[test]
fn test_captures_len_set_expression() {
    let re = Regex::new(r"[a&&b]").unwrap();
    let _ = re.captures_len();
}

