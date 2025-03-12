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
    
    let mut buf = String::new();
    let mut formatter = fmt::Formatter::new(&mut buf);
    let visitor = TestVisitor { expecting: "" };
    
    let _ = visitor.expecting(&mut formatter);
}

#[test]
fn test_expecting_small_string() {
    struct TestVisitor {
        expecting: &'static str,
    }
    impl<'de> Visitor<'de> for TestVisitor {
        type Value = ();
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str(self.expecting)
        }
    }
    
    let mut buf = String::new();
    let mut formatter = fmt::Formatter::new(&mut buf);
    let visitor = TestVisitor { expecting: "test" };
    
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
    
    let mut buf = String::new();
    let mut formatter = fmt::Formatter::new(&mut buf);
    let visitor = TestVisitor { expecting: "a".repeat(255).as_str() };
    
    let _ = visitor.expecting(&mut formatter);
}

