// Answer 0

#[test]
fn test_static_captures_len_empty() {
    let regex = Regex::new("").unwrap();
    let _ = regex.static_captures_len();
}

#[test]
fn test_static_captures_len_single_character() {
    let regex = Regex::new("a").unwrap();
    let _ = regex.static_captures_len();
}

#[test]
fn test_static_captures_len_single_capture() {
    let regex = Regex::new("(a)").unwrap();
    let _ = regex.static_captures_len();
}

#[test]
fn test_static_captures_len_alternation_with_single_capture() {
    let regex = Regex::new("(a)|(b)").unwrap();
    let _ = regex.static_captures_len();
}

#[test]
fn test_static_captures_len_multiple_captures() {
    let regex = Regex::new("(a)(b)|(c)(d)").unwrap();
    let _ = regex.static_captures_len();
}

#[test]
fn test_static_captures_len_alternation_without_capture() {
    let regex = Regex::new("(a)|b").unwrap();
    let _ = regex.static_captures_len();
}

#[test]
fn test_static_captures_len_alternation_with_capture() {
    let regex = Regex::new("a|(b)").unwrap();
    let _ = regex.static_captures_len();
}

#[test]
fn test_static_captures_len_zero_or_more() {
    let regex = Regex::new("(b)*").unwrap();
    let _ = regex.static_captures_len();
}

#[test]
fn test_static_captures_len_one_or_more() {
    let regex = Regex::new("(b)+").unwrap();
    let _ = regex.static_captures_len();
}

