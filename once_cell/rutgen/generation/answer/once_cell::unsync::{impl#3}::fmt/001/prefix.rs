// Answer 0

#[test]
fn test_fmt_with_initialized_value() {
    struct TestValue {
        data: i32,
    }

    impl fmt::Debug for TestValue {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "TestValue({})", self.data)
        }
    }

    let value = TestValue { data: 42 };
    let cell = OnceCell::with_value(value);
    let _ = fmt::Debug::fmt(&cell, &mut fmt::Formatter::new());
}

#[test]
fn test_fmt_with_initialized_different_value() {
    struct AnotherTestValue {
        name: String,
    }

    impl fmt::Debug for AnotherTestValue {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "AnotherTestValue({})", self.name)
        }
    }

    let value = AnotherTestValue { name: "Test".to_string() };
    let cell = OnceCell::with_value(value);
    let _ = fmt::Debug::fmt(&cell, &mut fmt::Formatter::new());
}

