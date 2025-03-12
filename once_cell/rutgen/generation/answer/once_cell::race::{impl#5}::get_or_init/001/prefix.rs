// Answer 0

#[test]
fn test_get_or_init_with_uninitialized_inner() {
    struct TestData {
        value: i32,
    }

    let once_ref: OnceRef<TestData> = OnceRef::new();
    let value_provider = || {
        let data = TestData { value: 42 };
        &data
    };

    once_ref.get_or_init(value_provider);
}

#[test]
#[should_panic] // This test will panic if the inner pointer is already initialized.
fn test_get_or_init_when_already_initialized() {
    struct TestData {
        value: i32,
    }

    let once_ref: OnceRef<TestData> = OnceRef::new();
    let initial_data = TestData { value: 100 };
    // Simulating the set function directly to initialize the inner pointer
    let _ = once_ref.set(&initial_data);

    let value_provider = || {
        let data = TestData { value: 21 };
        &data
    };

    once_ref.get_or_init(value_provider);
}

