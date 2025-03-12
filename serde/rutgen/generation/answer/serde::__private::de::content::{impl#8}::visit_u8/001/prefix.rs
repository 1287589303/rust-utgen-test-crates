// Answer 0

#[test]
fn test_visit_u8_zero() {
    struct TestVisitor;

    let visitor = TestVisitor;
    let _ = visitor.visit_u8(0);
}

#[test]
fn test_visit_u8_one() {
    struct TestVisitor;

    let visitor = TestVisitor;
    let _ = visitor.visit_u8(1);
}

#[test]
fn test_visit_u8_middle() {
    struct TestVisitor;

    let visitor = TestVisitor;
    let _ = visitor.visit_u8(128);
}

#[test]
fn test_visit_u8_max() {
    struct TestVisitor;

    let visitor = TestVisitor;
    let _ = visitor.visit_u8(255);
}

