// Answer 0

#[test]
fn test_visit_bool_true() {
    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = ();
        fn expecting(&self, _: &mut fmt::Formatter) -> fmt::Result {
            Ok(())
        }
    }

    let visitor = TestVisitor;
    let result: Result<(), _> = visitor.visit_bool(true);
}

#[test]
fn test_visit_bool_false() {
    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = ();
        fn expecting(&self, _: &mut fmt::Formatter) -> fmt::Result {
            Ok(())
        }
    }

    let visitor = TestVisitor;
    let result: Result<(), _> = visitor.visit_bool(false);
}

