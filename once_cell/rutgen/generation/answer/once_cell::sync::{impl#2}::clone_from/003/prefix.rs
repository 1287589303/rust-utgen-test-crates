// Answer 0

#[test]
fn test_clone_from_with_initialized_values() {
    struct TestStruct {
        value: i32,
    }

    impl Clone for TestStruct {
        fn clone(&self) -> Self {
            TestStruct { value: self.value }
        }
    }

    let cell_a = OnceCell::with_value(TestStruct { value: 10 });
    let mut cell_b = OnceCell::new();
    
    if let Some(value) = cell_a.get() {
        let _ = cell_b.set(value.clone());
    }
    
    cell_b.clone_from(&cell_a);
}

#[test]
fn test_clone_from_with_same_type() {
    struct TestType {
        data: String,
    }

    impl Clone for TestType {
        fn clone(&self) -> Self {
            TestType { data: self.data.clone() }
        }
    }

    let cell_x = OnceCell::with_value(TestType { data: "Hello".to_string() });
    let mut cell_y = OnceCell::new();
    
    if let Some(value) = cell_x.get() {
        let _ = cell_y.set(value.clone());
    }
    
    cell_y.clone_from(&cell_x);
}

#[test]
fn test_clone_from_after_set() {
    struct AnotherStruct {
        number: usize,
    }

    impl Clone for AnotherStruct {
        fn clone(&self) -> Self {
            AnotherStruct { number: self.number }
        }
    }

    let cell_1 = OnceCell::with_value(AnotherStruct { number: 5 });
    let mut cell_2 = OnceCell::new();

    if let Some(value) = cell_1.get() {
        let _ = cell_2.set(value.clone());
    }

    cell_2.clone_from(&cell_1);
}

