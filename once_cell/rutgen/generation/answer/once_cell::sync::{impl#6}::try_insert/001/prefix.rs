// Answer 0

#[test]
fn test_try_insert_err_with_different_value() {
    struct TestType {
        value: i32,
    }

    let cell = OnceCell::new();
    let _ = cell.try_insert(TestType { value: 42 });

    let result = cell.try_insert(TestType { value: 62 });
    let expected_value = TestType { value: 62 };
    
    let _ = result; // This should be Err((&TestType { value: 42 }, TestType { value: 62 }));
}

#[test]
fn test_try_insert_err_with_non_copy_type() {
    struct NonCopyType {
        value: Vec<i32>,
    }

    let cell = OnceCell::new();
    let _ = cell.try_insert(NonCopyType { value: vec![1, 2, 3] });

    let result = cell.try_insert(NonCopyType { value: vec![4, 5, 6] });
    let expected_value = NonCopyType { value: vec![4, 5, 6] };
    
    let _ = result; // This should also be Err((&NonCopyType { value: vec![1, 2, 3] }, NonCopyType { value: vec![4, 5, 6] }));
}

