// Answer 0

#[test]
fn test_visit_i8_with_min_value() {
    struct TestVisitor;
    impl<'de> Visitor<'de> for TestVisitor {
        type Value = ();
        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("an i8")
        }
    }

    let visitor = TestVisitor;
    let result = visitor.visit_i8(-128);
}

#[test]
fn test_visit_i8_with_zero() {
    struct TestVisitor;
    impl<'de> Visitor<'de> for TestVisitor {
        type Value = ();
        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("an i8")
        }
    }

    let visitor = TestVisitor;
    let result = visitor.visit_i8(0);
}

#[test]
fn test_visit_i8_with_positive_value() {
    struct TestVisitor;
    impl<'de> Visitor<'de> for TestVisitor {
        type Value = ();
        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("an i8")
        }
    }

    let visitor = TestVisitor;
    let result = visitor.visit_i8(127);
}

#[test]
#[should_panic]
fn test_visit_i8_with_below_min_value() {
    struct TestVisitor;
    impl<'de> Visitor<'de> for TestVisitor {
        type Value = ();
        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("an i8")
        }
    }

    let visitor = TestVisitor;
    let result = visitor.visit_i8(-129);
}

#[test]
#[should_panic]
fn test_visit_i8_with_above_max_value() {
    struct TestVisitor;
    impl<'de> Visitor<'de> for TestVisitor {
        type Value = ();
        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("an i8")
        }
    }

    let visitor = TestVisitor;
    let result = visitor.visit_i8(128);
}

