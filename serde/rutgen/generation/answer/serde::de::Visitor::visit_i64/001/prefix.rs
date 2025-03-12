// Answer 0

#[test]
fn test_visit_i64_negative_min() {
    struct TestVisitor;
    impl<'de> Visitor<'de> for TestVisitor {
        type Value = ();
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("test")
        }
    }

    let _ = TestVisitor.visit_i64(-9223372036854775808);
}

#[test]
fn test_visit_i64_negative_one() {
    struct TestVisitor;
    impl<'de> Visitor<'de> for TestVisitor {
        type Value = ();
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("test")
        }
    }

    let _ = TestVisitor.visit_i64(-1);
}

#[test]
fn test_visit_i64_zero() {
    struct TestVisitor;
    impl<'de> Visitor<'de> for TestVisitor {
        type Value = ();
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("test")
        }
    }

    let _ = TestVisitor.visit_i64(0);
}

#[test]
fn test_visit_i64_one() {
    struct TestVisitor;
    impl<'de> Visitor<'de> for TestVisitor {
        type Value = ();
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("test")
        }
    }

    let _ = TestVisitor.visit_i64(1);
}

#[test]
fn test_visit_i64_positive_max() {
    struct TestVisitor;
    impl<'de> Visitor<'de> for TestVisitor {
        type Value = ();
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("test")
        }
    }

    let _ = TestVisitor.visit_i64(9223372036854775807);
}

