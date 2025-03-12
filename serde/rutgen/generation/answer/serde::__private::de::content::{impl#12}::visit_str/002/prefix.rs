// Answer 0

#[test]
fn test_visit_str_equals_content() {
    struct TestVisitor {
        tag: &'static str,
        content: &'static str,
    }

    let visitor = TestVisitor {
        tag: "not_tag",
        content: "content_value",
    };
    
    let result = visitor.visit_str("content_value");
}

#[test]
fn test_visit_str_not_equals_tag_equals_content() {
    struct TestVisitor {
        tag: &'static str,
        content: &'static str,
    }

    let visitor = TestVisitor {
        tag: "tag_value",
        content: "content_value",
    };

    let result = visitor.visit_str("content_value");
}

#[test]
fn test_visit_str_different_cases() {
    struct TestVisitor {
        tag: &'static str,
        content: &'static str,
    }

    let visitor = TestVisitor {
        tag: "TAG",
        content: "Content",
    };

    let result = visitor.visit_str("Content");
}

