// Answer 0

#[test]
fn test_set_with_unique_value() {
    struct MyValue(i32);
    let cell = OnceCell::new();
    
    let value1 = MyValue(0);
    assert_eq!(cell.set(value1), Ok(()));
}

#[test]
fn test_set_with_another_unique_value() {
    struct MyValue(i32);
    let cell = OnceCell::new();
    
    let value2 = MyValue(1);
    assert_eq!(cell.set(value2), Ok(()));
}

#[test]
fn test_set_with_zero_value() {
    struct MyValue(i32);
    let cell = OnceCell::new();

    let value0 = MyValue(0);
    assert_eq!(cell.set(value0), Ok(()));
}

#[test]
fn test_set_with_different_unique_object() {
    struct MyValue(i32);
    let cell = OnceCell::new();
    
    let value3 = MyValue(2);
    assert_eq!(cell.set(value3), Ok(()));
}

#[test]
fn test_set_with_large_value() {
    struct MyValue(i32);
    let cell = OnceCell::new();
    
    let large_value = MyValue(9999);
    assert_eq!(cell.set(large_value), Ok(()));
}

