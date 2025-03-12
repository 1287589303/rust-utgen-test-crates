// Answer 0

#[test]
fn test_visit_str_non_empty() {
    let mut output = String::new();
    let visitor = StringInPlaceVisitor(&mut output);
    let input = "Hello, World!";
    let _ = visitor.visit_str(input);
}

#[test]
fn test_visit_str_empty() {
    let mut output = String::new();
    let visitor = StringInPlaceVisitor(&mut output);
    let input = "";
    let _ = visitor.visit_str(input);
}

