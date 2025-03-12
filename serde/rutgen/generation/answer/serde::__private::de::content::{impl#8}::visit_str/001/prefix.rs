// Answer 0

#[test]
fn test_visit_str_equal_name_empty() {
    struct TestVisitor {
        name: &'static str,
    }
    
    let visitor = TestVisitor { name: "" };
    let result = visitor.visit_str("");
}

#[test]
fn test_visit_str_equal_name_typical() {
    struct TestVisitor {
        name: &'static str,
    }
    
    let visitor = TestVisitor { name: "example" };
    let result = visitor.visit_str("example");
}

#[test]
fn test_visit_str_equal_name_long_string() {
    struct TestVisitor {
        name: &'static str,
    }
    
    let long_name = "a".repeat(1000);
    let visitor = TestVisitor { name: &long_name };
    let result = visitor.visit_str(&long_name);
} 

#[test]
fn test_visit_str_equal_name_special_chars() {
    struct TestVisitor {
        name: &'static str,
    }
    
    let visitor = TestVisitor { name: "!@#$%^&*()" };
    let result = visitor.visit_str("!@#$%^&*()");
} 

#[test]
fn test_visit_str_equal_name_with_spaces() {
    struct TestVisitor {
        name: &'static str,
    }
    
    let visitor = TestVisitor { name: "name with spaces" };
    let result = visitor.visit_str("name with spaces");
}

