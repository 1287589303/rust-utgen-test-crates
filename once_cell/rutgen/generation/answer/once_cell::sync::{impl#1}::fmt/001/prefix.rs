// Answer 0

#[test]
fn test_fmt_with_initialized_value() {
    struct TestData {
        value: i32,
    }
    
    impl fmt::Debug for TestData {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "TestData({})", self.value)
        }
    }
    
    let cell = OnceCell::with_value(TestData { value: 42 });
    let mut formatter = fmt::Formatter::new();
    let _ = cell.fmt(&mut formatter);
}

#[test]
fn test_fmt_with_set_value() {
    struct TestData {
        value: i32,
    }
    
    impl fmt::Debug for TestData {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "TestData({})", self.value)
        }
    }
    
    let cell = OnceCell::new();
    let _ = cell.set(TestData { value: 24 });
    let mut formatter = fmt::Formatter::new();
    let _ = cell.fmt(&mut formatter);
}

