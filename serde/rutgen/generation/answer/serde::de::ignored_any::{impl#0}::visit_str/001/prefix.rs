// Answer 0

#[test]
fn test_visit_str_empty() {
    let visitor = IgnoredAny;
    let input: &str = "";
    let _ = visitor.visit_str(input);
}

#[test]
fn test_visit_str_one_char() {
    let visitor = IgnoredAny;
    let input: &str = "a";
    let _ = visitor.visit_str(input);
}

#[test]
fn test_visit_str_special_char() {
    let visitor = IgnoredAny;
    let input: &str = "😊";
    let _ = visitor.visit_str(input);
}

#[test]
fn test_visit_str_long() {
    let visitor = IgnoredAny;
    let input: &str = "a".repeat(2u16.pow(16));
    let _ = visitor.visit_str(&input);
}

