// Answer 0

#[test]
fn test_capture_names_with_mixed_groups() {
    let re = Regex::new(r"(?<a>.(?<b>.))(.)(?:.)(?<c>.)").unwrap();
    let mut names = re.capture_names();
    let _ = names.next(); // Should yield Some(None)
    let _ = names.next(); // Should yield Some(Some("a"))
    let _ = names.next(); // Should yield Some(Some("b"))
    let _ = names.next(); // Should yield Some(None)
    let _ = names.next(); // Should yield Some(Some("c"))
    let _ = names.next(); // Should yield None
}

#[test]
fn test_capture_names_with_empty_regex() {
    let re = Regex::new(r"").unwrap();
    let mut names = re.capture_names();
    let _ = names.next(); // Should yield Some(None)
    let _ = names.next(); // Should yield None
}

#[test]
fn test_capture_names_with_no_capture_groups() {
    let re = Regex::new(r"[a&&b]").unwrap();
    let mut names = re.capture_names();
    let _ = names.next(); // Should yield Some(None)
    let _ = names.next(); // Should yield None
}

#[test]
fn test_capture_names_with_single_character() {
    let re = Regex::new(r"(a)").unwrap();
    let mut names = re.capture_names();
    let _ = names.next(); // Should yield Some(None)
    let _ = names.next(); // Should yield None
}

#[test]
fn test_capture_names_with_multiple_consecutive_characters() {
    let re = Regex::new(r"(abc)(def)").unwrap();
    let mut names = re.capture_names();
    let _ = names.next(); // Should yield Some(None)
    let _ = names.next(); // Should yield None
}

#[test]
fn test_capture_names_with_non_capturing_group() {
    let re = Regex::new(r"(?:abc)(def)").unwrap();
    let mut names = re.capture_names();
    let _ = names.next(); // Should yield Some(None)
    let _ = names.next(); // Should yield None
}

