// Answer 0

#[test]
fn test_expecting_with_non_empty_name() {
    struct TestVisitor {
        name: &'static str,
    }

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = ();
        fn expecting(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(fmt, "a type tag `{}` or any other value", self.name)
        }
    }

    let visitor = TestVisitor { name: "test_name" };
    let mut buffer = String::new();
    let fmt = &mut std::fmt::Formatter::new(&mut buffer);
    let _ = visitor.expecting(fmt);
}

#[test]
fn test_expecting_with_empty_name() {
    struct TestVisitor {
        name: &'static str,
    }

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = ();
        fn expecting(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(fmt, "a type tag `{}` or any other value", self.name)
        }
    }

    let visitor = TestVisitor { name: "" };
    let mut buffer = String::new();
    let fmt = &mut std::fmt::Formatter::new(&mut buffer);
    let _ = visitor.expecting(fmt);
}

