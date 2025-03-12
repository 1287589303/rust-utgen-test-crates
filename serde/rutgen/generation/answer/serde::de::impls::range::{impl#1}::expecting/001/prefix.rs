// Answer 0

#[test]
fn test_expecting_empty_string() {
    struct TestVisitor {
        expecting: &'static str,
    }

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = ();
        
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str(self.expecting)
        }
    }

    let visitor = TestVisitor { expecting: "" };
    let mut formatter = std::fmt::Formatter::new();
    let _ = visitor.expecting(&mut formatter);
}

#[test]
fn test_expecting_normal_string() {
    struct TestVisitor {
        expecting: &'static str,
    }

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = ();
        
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str(self.expecting)
        }
    }

    let visitor = TestVisitor { expecting: "Expected format" };
    let mut formatter = std::fmt::Formatter::new();
    let _ = visitor.expecting(&mut formatter);
}

#[test]
fn test_expecting_large_string() {
    struct TestVisitor {
        expecting: &'static str,
    }

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = ();
        
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str(self.expecting)
        }
    }

    let large_string = "a".repeat(1000); // A long string.
    let visitor = TestVisitor { expecting: &large_string };
    let mut formatter = std::fmt::Formatter::new();
    let _ = visitor.expecting(&mut formatter);
}

