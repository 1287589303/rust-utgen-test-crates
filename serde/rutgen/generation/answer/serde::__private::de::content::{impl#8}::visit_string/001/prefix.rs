// Answer 0

#[test]
fn test_visit_string_equals_name() {
    struct TestVisitor {
        name: &'static str,
    }

    let visitor = TestVisitor { name: "test" };
    let input_value = String::from("test");

    let _result: TagOrContent = visitor.visit_string(input_value).unwrap();
}

#[test]
fn test_visit_string_equals_name_with_different_case() {
    struct TestVisitor {
        name: &'static str,
    }

    let visitor = TestVisitor { name: "test" };
    let input_value = String::from("TEST");

    let _result: Result<TagOrContent, Box<dyn de::Error>> = visitor.visit_string(input_value);
}

#[test]
fn test_visit_string_equals_name_with_spaces() {
    struct TestVisitor {
        name: &'static str,
    }

    let visitor = TestVisitor { name: "test " };
    let input_value = String::from("test ");

    let _result: TagOrContent = visitor.visit_string(input_value).unwrap();
}

