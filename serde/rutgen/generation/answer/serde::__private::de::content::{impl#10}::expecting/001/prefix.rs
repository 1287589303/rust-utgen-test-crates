// Answer 0

#[test]
fn test_expecting_valid_string() {
    struct TestVisitor {
        expecting: &'static str,
    }

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = ();
        fn expecting(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
            fmt.write_str(self.expecting)
        }
    }

    let mut formatter = String::new();
    let visitor = TestVisitor { expecting: "valid string" };
    visitor.expecting(&mut formatter);
}

#[test]
fn test_expecting_empty_string() {
    struct TestVisitor {
        expecting: &'static str,
    }

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = ();
        fn expecting(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
            fmt.write_str(self.expecting)
        }
    }

    let mut formatter = String::new();
    let visitor = TestVisitor { expecting: "" };
    visitor.expecting(&mut formatter);
}

#[test]
fn test_expecting_multiple_words() {
    struct TestVisitor {
        expecting: &'static str,
    }

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = ();
        fn expecting(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
            fmt.write_str(self.expecting)
        }
    }

    let mut formatter = String::new();
    let visitor = TestVisitor {
        expecting: "multiple words string",
    };
    visitor.expecting(&mut formatter);
}

