// Answer 0

#[test]
fn test_visit_f64_positive() {
    struct TestVisitor;
    impl<'de> Visitor<'de> for TestVisitor {
        type Value = ();
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            write!(formatter, "a float")
        }
    }
    let visitor = TestVisitor;
    let _ = visitor.visit_f64(1.0);
}

#[test]
fn test_visit_f64_negative() {
    struct TestVisitor;
    impl<'de> Visitor<'de> for TestVisitor {
        type Value = ();
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            write!(formatter, "a float")
        }
    }
    let visitor = TestVisitor;
    let _ = visitor.visit_f64(-1.0);
}

#[test]
fn test_visit_f64_zero() {
    struct TestVisitor;
    impl<'de> Visitor<'de> for TestVisitor {
        type Value = ();
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            write!(formatter, "a float")
        }
    }
    let visitor = TestVisitor;
    let _ = visitor.visit_f64(0.0);
}

#[test]
fn test_visit_f64_infinity() {
    struct TestVisitor;
    impl<'de> Visitor<'de> for TestVisitor {
        type Value = ();
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            write!(formatter, "a float")
        }
    }
    let visitor = TestVisitor;
    let _ = visitor.visit_f64(f64::INFINITY);
}

#[test]
fn test_visit_f64_negative_infinity() {
    struct TestVisitor;
    impl<'de> Visitor<'de> for TestVisitor {
        type Value = ();
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            write!(formatter, "a float")
        }
    }
    let visitor = TestVisitor;
    let _ = visitor.visit_f64(f64::NEG_INFINITY);
}

#[test]
fn test_visit_f64_nan() {
    struct TestVisitor;
    impl<'de> Visitor<'de> for TestVisitor {
        type Value = ();
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            write!(formatter, "a float")
        }
    }
    let visitor = TestVisitor;
    let _ = visitor.visit_f64(f64::NAN);
}

