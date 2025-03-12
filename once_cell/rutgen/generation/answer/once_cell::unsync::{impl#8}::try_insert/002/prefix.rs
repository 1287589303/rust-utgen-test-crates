// Answer 0

#[test]
fn test_try_insert_with_existing_value() {
    struct TestStruct {
        value: i32,
    }
    
    let cell = OnceCell::new();
    let _ = cell.try_insert(TestStruct { value: 42 }); // First insertion should succeed.
    let result = cell.try_insert(TestStruct { value: 99 }); // Second insertion should fail with Err.

    let _ = result; // Here we expect Err((&TestStruct { value: 42 }, TestStruct { value: 99 }))
}

#[test]
fn test_try_insert_with_same_value() {
    struct TestStruct {
        value: i32,
    }
    
    let cell = OnceCell::new();
    let _ = cell.try_insert(TestStruct { value: 42 }); // First insertion should succeed.
    let result = cell.try_insert(TestStruct { value: 42 }); // Second insertion with the same value should fail.

    let _ = result; // Here we expect Err((&TestStruct { value: 42 }, TestStruct { value: 42 }))
}

#[test]
fn test_try_insert_after_multiple_insertions() {
    struct TestStruct {
        value: i32,
    }
    
    let cell = OnceCell::new();
    let _ = cell.try_insert(TestStruct { value: 10 }); // First insertion should succeed.
    let _ = cell.try_insert(TestStruct { value: 20 }); // Second insertion should also succeed.
    
    let result = cell.try_insert(TestStruct { value: 30 }); // Third insertion should fail with Err.

    let _ = result; // Here we expect Err((&TestStruct { value: 20 }, TestStruct { value: 30 }))
}

