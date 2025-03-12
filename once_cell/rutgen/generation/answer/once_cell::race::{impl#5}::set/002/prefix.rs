// Answer 0

#[test]
fn test_set_success() {
    struct TestValue;
    let value = TestValue;
    let once_ref: OnceRef<TestValue> = OnceRef::new();
    
    let result = once_ref.set(&value);
}

#[test]
fn test_set_success_with_different_value() {
    struct TestValue;
    let value1 = TestValue;
    let value2 = TestValue;
    let once_ref: OnceRef<TestValue> = OnceRef::new();
    
    let result1 = once_ref.set(&value1);
    let result2 = once_ref.set(&value2);
}

