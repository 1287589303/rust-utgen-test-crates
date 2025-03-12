// Answer 0

#[test]
fn test_visit_f64_negative_one() {
    struct TestVisitor;
    let visitor = TestVisitor;
    let _ = visitor.visit_f64(-1.0);
}

#[test]
fn test_visit_f64_zero() {
    struct TestVisitor;
    let visitor = TestVisitor;
    let _ = visitor.visit_f64(0.0);
}

#[test]
fn test_visit_f64_one() {
    struct TestVisitor;
    let visitor = TestVisitor;
    let _ = visitor.visit_f64(1.0);
}

#[test]
fn test_visit_f64_min() {
    struct TestVisitor;
    let visitor = TestVisitor;
    let _ = visitor.visit_f64(std::f64::MIN);
}

#[test]
fn test_visit_f64_max() {
    struct TestVisitor;
    let visitor = TestVisitor;
    let _ = visitor.visit_f64(std::f64::MAX);
}

#[test]
fn test_visit_f64_nan() {
    struct TestVisitor;
    let visitor = TestVisitor;
    let _ = visitor.visit_f64(std::f64::NAN);
}

#[test]
fn test_visit_f64_infinity() {
    struct TestVisitor;
    let visitor = TestVisitor;
    let _ = visitor.visit_f64(std::f64::INFINITY);
}

#[test]
fn test_visit_f64_neg_infinity() {
    struct TestVisitor;
    let visitor = TestVisitor;
    let _ = visitor.visit_f64(std::f64::NEG_INFINITY);
}

