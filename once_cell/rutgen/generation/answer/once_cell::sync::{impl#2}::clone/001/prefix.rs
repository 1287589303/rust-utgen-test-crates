// Answer 0

#[test]
fn test_clone_with_value() {
    struct TestValue {
        data: i32,
    }
    impl Clone for TestValue {
        fn clone(&self) -> Self {
            TestValue { data: self.data }
        }
    }

    let original = OnceCell::with_value(TestValue { data: 42 });
    let cloned = original.clone();
}

#[test]
fn test_clone_with_string() {
    let original = OnceCell::with_value("Hello".to_string());
    let cloned = original.clone();
}

#[test]
fn test_clone_with_vector() {
    let original = OnceCell::with_value(vec![1, 2, 3]);
    let cloned = original.clone();
}

