// Answer 0

#[test]
fn test_visit_u16_min() {
    struct TestVisitor;
    impl<'de> Visitor<'de> for TestVisitor {
        type Value = TagOrContent<'de>;
        fn expecting(&self, _: &mut fmt::Formatter) -> fmt::Result {
            Ok(())
        }
    }
    let visitor = TestVisitor;
    let result = visitor.visit_u16(0u16);
}

#[test]
fn test_visit_u16_max() {
    struct TestVisitor;
    impl<'de> Visitor<'de> for TestVisitor {
        type Value = TagOrContent<'de>;
        fn expecting(&self, _: &mut fmt::Formatter) -> fmt::Result {
            Ok(())
        }
    }
    let visitor = TestVisitor;
    let result = visitor.visit_u16(65535u16);
}

#[test]
fn test_visit_u16_just_below_max() {
    struct TestVisitor;
    impl<'de> Visitor<'de> for TestVisitor {
        type Value = TagOrContent<'de>;
        fn expecting(&self, _: &mut fmt::Formatter) -> fmt::Result {
            Ok(())
        }
    }
    let visitor = TestVisitor;
    let result = visitor.visit_u16(65534u16);
}

#[should_panic]
#[test]
fn test_visit_u16_above_max() {
    struct TestVisitor;
    impl<'de> Visitor<'de> for TestVisitor {
        type Value = TagOrContent<'de>;
        fn expecting(&self, _: &mut fmt::Formatter) -> fmt::Result {
            Ok(())
        }
    }
    let visitor = TestVisitor;
    let result = visitor.visit_u16(65536u16);
}

