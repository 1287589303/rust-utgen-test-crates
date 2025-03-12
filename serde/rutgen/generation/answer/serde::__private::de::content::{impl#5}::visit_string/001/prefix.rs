// Answer 0

#[test]
fn test_visit_string_valid_non_empty() {
    let visitor = ContentVisitor { value: PhantomData };
    let input = String::from("valid string");
    let _ = visitor.visit_string(input);
}

#[test]
fn test_visit_string_valid_empty() {
    let visitor = ContentVisitor { value: PhantomData };
    let input = String::from("");
    let _ = visitor.visit_string(input);
}

#[test]
fn test_visit_string_very_long() {
    let visitor = ContentVisitor { value: PhantomData };
    let input = String::from("a".repeat(1_000_000)); // very long string
    let _ = visitor.visit_string(input);
}

