// Answer 0

#[test]
fn test_clone_from_with_cloned_value() {
    struct TestData {
        value: i32,
    }

    impl Clone for TestData {
        fn clone(&self) -> Self {
            TestData { value: self.value }
        }
    }

    let mut cell1 = OnceCell::new();
    let cell2 = OnceCell::with_value(TestData { value: 42 });

    // Set cell1 with a cloned value from cell2
    cell1.set(TestData { value: 42 }).unwrap();

    // Clone cell2 into cell1; asserts that both cells have valid mutable references
    cell1.clone_from(&cell2);
}

#[test]
fn test_clone_from_with_different_values() {
    struct TestData {
        value: i32,
    }

    impl Clone for TestData {
        fn clone(&self) -> Self {
            TestData { value: self.value }
        }
    }

    let mut cell1 = OnceCell::new();
    let cell2 = OnceCell::with_value(TestData { value: 100 });

    // Set cell1 with a different value
    cell1.set(TestData { value: 50 }).unwrap();

    // Clone cell2 into cell1; asserts that both cells have valid mutable references
    cell1.clone_from(&cell2);
}

