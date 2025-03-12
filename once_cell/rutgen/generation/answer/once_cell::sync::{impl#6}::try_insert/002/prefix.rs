// Answer 0

#[test]
fn test_try_insert_unique_value() {
    struct TestType(i32);
    
    let cell = OnceCell::new();
    assert!(cell.get().is_none());
    
    let value = TestType(100);
    let result = cell.try_insert(value);
}

#[test]
fn test_try_insert_with_different_unique_value() {
    struct TestType(i32);
    
    let cell = OnceCell::new();
    assert!(cell.get().is_none());
    
    let value1 = TestType(200);
    let result1 = cell.try_insert(value1);
    
    // The cell should now have the first value set
    let value2 = TestType(300);
    let result2 = cell.try_insert(value2);
}

