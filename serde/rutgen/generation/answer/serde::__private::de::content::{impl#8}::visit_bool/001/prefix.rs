// Answer 0

#[test]
fn test_visit_bool_true() {
    struct TestVisitor;
    impl<'de> Visitor<'de> for TestVisitor {
        type Value = TagOrContent<'de>;
        fn expecting(&self, _: &mut std::fmt::Formatter) -> std::fmt::Result {
            Ok(())
        }
    }

    let visitor = TestVisitor;
    let result = visitor.visit_bool(true);
}

#[test]
fn test_visit_bool_false() {
    struct TestVisitor;
    impl<'de> Visitor<'de> for TestVisitor {
        type Value = TagOrContent<'de>;
        fn expecting(&self, _: &mut std::fmt::Formatter) -> std::fmt::Result {
            Ok(())
        }
    }

    let visitor = TestVisitor;
    let result = visitor.visit_bool(false);
}

