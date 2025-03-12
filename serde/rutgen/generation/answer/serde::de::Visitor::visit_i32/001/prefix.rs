// Answer 0

#[test]
fn test_visit_i32_min() {
    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = ();
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result { Ok(()) }
    }

    let visitor = TestVisitor;
    let _ = visitor.visit_i32::<()>(i32::min_value());
}

#[test]
fn test_visit_i32_negative() {
    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = ();
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result { Ok(()) }
    }

    let visitor = TestVisitor;
    let _ = visitor.visit_i32::<()>(-1);
}

#[test]
fn test_visit_i32_zero() {
    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = ();
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result { Ok(()) }
    }

    let visitor = TestVisitor;
    let _ = visitor.visit_i32::<()>(0);
}

#[test]
fn test_visit_i32_positive() {
    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = ();
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result { Ok(()) }
    }

    let visitor = TestVisitor;
    let _ = visitor.visit_i32::<()>(1);
}

#[test]
fn test_visit_i32_max() {
    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = ();
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result { Ok(()) }
    }

    let visitor = TestVisitor;
    let _ = visitor.visit_i32::<()>(i32::max_value());
}

